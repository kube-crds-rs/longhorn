#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
/// Setting is where Longhorn stores setting object.
pub struct Setting {
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub value: String,
}

impl k8s_openapi::Resource for Setting {
    const API_VERSION: &'static str = "longhorn.io/v1beta2";

    const GROUP: &'static str = "longhorn.io";
    const KIND: &'static str = "Setting";
    const VERSION: &'static str = "v1beta2";
    const URL_PATH_SEGMENT: &'static str = "settings";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl k8s_openapi::Metadata for Setting {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl serde::Serialize for Setting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Setting", 4)?;
        state.serialize_field("apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        state.serialize_field("kind", <Self as k8s_openapi::Resource>::KIND)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

