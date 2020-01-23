initSidebarItems({"enum":[["CreateRuntimeClassResponse","Use `<CreateRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::create_runtime_class`]"],["DeleteCollectionRuntimeClassResponse","Use `<DeleteCollectionRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::delete_collection_runtime_class`]"],["DeleteRuntimeClassResponse","Use `<DeleteRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::delete_runtime_class`]"],["ListRuntimeClassResponse","Use `<ListRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::list_runtime_class`]"],["PatchRuntimeClassResponse","Use `<PatchRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::patch_runtime_class`]"],["ReadRuntimeClassResponse","Use `<ReadRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::read_runtime_class`]"],["ReplaceRuntimeClassResponse","Use `<ReplaceRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::replace_runtime_class`]"],["WatchRuntimeClassResponse","Use `<WatchRuntimeClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`RuntimeClass::watch_runtime_class`]"]],"struct":[["CreateRuntimeClassOptional","Optional parameters of [`RuntimeClass::create_runtime_class`]"],["Overhead","Overhead structure represents the resource overhead associated with running a pod."],["ReadRuntimeClassOptional","Optional parameters of [`RuntimeClass::read_runtime_class`]"],["ReplaceRuntimeClassOptional","Optional parameters of [`RuntimeClass::replace_runtime_class`]"],["RuntimeClass","RuntimeClass defines a class of container runtime supported in the cluster. The RuntimeClass is used to determine which container runtime is used to run all containers in a pod. RuntimeClasses are (currently) manually defined by a user or cluster provisioner, and referenced in the PodSpec. The Kubelet is responsible for resolving the RuntimeClassName reference before running the pod.  For more details, see https://git.k8s.io/enhancements/keps/sig-node/runtime-class.md"],["RuntimeClassList","RuntimeClassList is a list of RuntimeClass objects."],["RuntimeClassSpec","RuntimeClassSpec is a specification of a RuntimeClass. It contains parameters that are required to describe the RuntimeClass to the Container Runtime Interface (CRI) implementation, as well as any other components that need to understand how the pod will be run. The RuntimeClassSpec is immutable."],["Scheduling","Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass."]]});