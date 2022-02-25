#![recursion_limit = "1024"]
#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::too_many_lines,
)]

//! This crate contains custom derives related to the [`k8s-openapi`](https://crates.io/crates/k8s-openapi) crate.

mod custom_resource_definition;

trait CustomDerive: Sized {
	fn parse(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<Self, syn::Error>;
	fn emit(self) -> Result<proc_macro2::TokenStream, syn::Error>;
}

fn run_custom_derive<T>(input: proc_macro::TokenStream) -> proc_macro::TokenStream where T: CustomDerive {
	let input: proc_macro2::TokenStream = input.into();
	let tokens = input.clone();
	let token_stream = match syn::parse2(input).and_then(|input| <T as CustomDerive>::parse(input, tokens)).and_then(<T as CustomDerive>::emit) {
		Ok(token_stream) => token_stream,
		Err(err) => err.to_compile_error(),
	};
	token_stream.into()
}

trait ResultExt<T> {
	fn spanning(self, spanned: impl quote::ToTokens) -> Result<T, syn::Error>;
}

impl<T, E> ResultExt<T> for Result<T, E> where E: std::fmt::Display {
	fn spanning(self, spanned: impl quote::ToTokens) -> Result<T, syn::Error> {
		self.map_err(|err| syn::Error::new_spanned(spanned, err))
	}
}

/// This custom derive can be used on a Kubernetes custom resource spec type to generate a custom resource definition object
/// and associated API functions.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(
///     Clone, Debug, PartialEq,
///     k8s_openapi_derive::CustomResourceDefinition,
///     serde::Deserialize, serde::Serialize,
/// )]
/// #[custom_resource_definition(
///     group = "k8s-openapi-tests-custom-resource-definition.com",
///     version = "v1",
///     plural = "foobars",
///     generate_schema,
///     namespaced,
///     has_subresources = "v1",
/// )]
/// struct FooBarSpec {
///     prop1: String,
///     prop2: Vec<bool>,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     prop3: Option<i32>,
/// }
/// ```
///
/// Note:
///
/// - The spec type must impl the following traits (either manually or via `#[derive]`): `Clone`, `Debug`, `PartialEq`,
///   `serde::Deserialize` and `serde::Serialize`
///
/// - The name of the spec type must end with `Spec`. This suffix is trimmed to generate the names of the other types.
///
/// - The `k8s_openapi` crate must have been added as a dependency, since the macro expansion refers to types from it.
///
/// The custom derive then generates a `FooBar` type that represents a custom resource corresponding to this definition:
///
/// ```rust,ignore
/// /// Custom resource for FooBarSpec
/// #[derive(Clone, Debug, Default, PartialEq)]
/// struct FooBar {
///     metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
///     spec: Option<FooBarSpec>,
///     subresources: k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
/// }
///
/// impl k8s_openapi::Resource for FooBar { ... }
///
/// impl k8s_openapi::ListableResource for FooBar { ... }
///
/// impl k8s_openapi::Metadata for FooBar {
///     type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
///
///     ...
/// }
///
/// impl<'de> k8s_openapi::serde::Deserialize<'de> for FooBar { ... }
///
/// impl k8s_openapi::serde::Serialize for FooBar { ... }
///
/// impl k8s_openapi::schemars::JsonSchema for FooBar { ... }
/// ```
///
/// The name of this type is automatically derived from the name of the spec type by truncating the `Spec` suffix.
///
/// The `group` and `version` meta items of the `#[custom_resource_definition]` attribute of the macro are used to set
/// the "group" and "API version" in the `k8s_openapi::Resource` impl respectively. The "kind" is automatically set to be the same as the resource type name,
/// ie `"FooBar"` in this example. The `plural` meta item is used to construct the URLs of API operations for this custom resource.
///
/// The `generate_schema` meta item is optional. If set, the generated custom resource type will have an impl of `schemars::JsonSchema` from the `schemars` crate.
/// The `schemars` feature of the `k8s-openapi` crate must be enabled so that the types in that crate also have their `schemars::JsonSchema` impls enabled.
/// You will also need to impl `schemars::JsonSchema` on the `Spec` type itself, either manually or via `#[derive(schemars::JsonSchema)]`.
///
/// The `has_subresources` meta item is optional. If set, the generated custom resource type will have a `subresources` field. The value of the meta item
/// specifies which namespace the type will be used from. For example, setting `has_subresources = "v1"` causes the field to be of the
/// `k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources` type.
///
/// You would then register this custom resource definition with Kubernetes, with code like this:
///
/// ```rust,ignore
/// use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions;
/// use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;
///
/// // Same as the `plurals` meta item in the `#[custom_resource_definition]` attribute
/// let plural = "foobars";
///
/// let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
///     group: <FooBar as k8s_openapi::Resource>::GROUP.to_owned(),
///     names: apiextensions::CustomResourceDefinitionNames {
///         kind: <FooBar as k8s_openapi::Resource>::KIND.to_owned(),
///         plural: plural.to_owned(),
///         short_names: Some(vec!["fb".to_owned()]),
///         singular: Some("foobar".to_owned()),
///         ..Default::default()
///     },
///     scope: "Namespaced".to_owned(),
///     version: <FooBar as k8s_openapi::Resource>::VERSION.to_owned().into(),
///     ..Default::default()
/// };
///
/// let custom_resource_definition = apiextensions::CustomResourceDefinition {
///     metadata: meta::ObjectMeta {
///         name: Some(format!("{plural}.{}", <FooBar as k8s_openapi::Resource>::GROUP)),
///         ..Default::default()
///     },
///     spec: custom_resource_definition_spec.into(),
///     ..Default::default()
/// };
///
/// let (request, response_body) =
///     apiextensions::CustomResourceDefinition::create_custom_resource_definition(&custom_resource_definition, Default::default())
///     .expect("couldn't create custom resource definition");
/// let response = client.execute(request).expect("couldn't create custom resource definition");
/// ```
///
/// The macro also generates clientset functions associated with the custom resource type to create, get, update, etc.
/// This is just like a regular Kubernetes resource type like `Pod`.
///
/// ```rust,ignore
/// impl FooBar {
///     /// Create a FooBar
///     fn create_namespaced_foo_bar(
///         namespace: &str,
///         body: &FooBar,
///         optional: k8s_openapi::DeleteOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Delete a FooBar
///     fn delete_namespaced_foo_bar(
///         name: &str,
///         namespace: &str,
///         optional: k8s_openapi::DeleteOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Delete a collection of objects of kind FooBar
///     fn delete_collection_namespaced_foo_bar(
///         namespace: &str,
///         delete_optional: k8s_openapi::DeleteOptional<'_>,
///         list_optional: k8s_openapi::ListOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// List objects of kind FooBar
///     fn list_namespaced_foo_bar(
///         namespace: &str,
///         optional: k8s_openapi::ListOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Partially update the specified FooBar
///     fn patch_namespaced_foo_bar(
///         name: &str,
///         namespace: &str,
///         body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
///         optional: k8s_openapi::PatchOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Partially update the state of the specified FooBar
///     fn patch_namespaced_foo_bar_status(
///         name: &str,
///         namespace: &str,
///         body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
///         optional: k8s_openapi::PatchOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Read the specified FooBar
///     fn read_namespaced_foo_bar(
///         name: &str,
///         namespace: &str,
///     ) -> Result<
///         (
///             k8s_openapi::http::Request<Vec<u8>>,
///             fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<ReadNamespacedFooBarResponse>
///         ),
///         k8s_openapi::RequestError,
///     > { ... }
///
///     /// Read status of the specified FooBar
///     fn read_namespaced_foo_bar_status(
///         name: &str,
///         namespace: &str,
///     ) -> Result<
///         (
///             k8s_openapi::http::Request<Vec<u8>>,
///             fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<ReadNamespacedFooBarStatusResponse>
///         ),
///         k8s_openapi::RequestError,
///     > { ... }
///
///     /// Replace the specified FooBar
///     fn replace_namespaced_foo_bar(
///         name: &str,
///         namespace: &str,
///         body: &FooBar,
///         optional: k8s_openapi::ReplaceOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Replace status of the specified FooBar
///     fn replace_namespaced_foo_bar_status(
///         name: &str,
///         namespace: &str,
///         body: &FooBar,
///         optional: k8s_openapi::ReplaceOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
///
///     /// Watch objects of kind FooBar
///     fn watch_namespaced_foo_bar(
///         namespace: &str,
///         optional: k8s_openapi::WatchOptional<'_>,
///     ) ->
///         Result<
///             (
///                 k8s_openapi::http::Request<Vec<u8>>,
///                 fn(k8s_openapi::http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>
///             ),
///             k8s_openapi::RequestError,
///         >
///     { ... }
/// }
///
/// /// FooBarList is a list of FooBar
/// type FooBarList = k8s_openapi::List<FooBar>;
/// ```
///
/// (You may wish to generate your own crate's docs, or run it through `cargo-expand`, to be able to see the macro expansion.)
///
/// Refer to [the `k8s-openapi` crate docs](https://arnavion.github.io/k8s-openapi/) to learn more about how to use the return values of these functions.
///
/// See the [`custom_resource_definition` test in the repository](https://github.com/Arnavion/k8s-openapi/blob/master/k8s-openapi-tests/src/custom_resource_definition.rs)
/// for a full example of using this custom derive.
#[proc_macro_derive(CustomResourceDefinition, attributes(custom_resource_definition))]
pub fn derive_custom_resource_definition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	run_custom_derive::<custom_resource_definition::CustomResourceDefinition>(input)
}
