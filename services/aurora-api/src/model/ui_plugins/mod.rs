use crate::web::bean::response::ui_plugin::UiPlugin;
use crate::{
    model::client::service::_ds_plugin_define_service_client, web::bean::response::ui_plugin::UiPluginNoParams,
};
use lib_common::core_results::results::Result;
use lib_proto::ds_plugin_define::{GetDsPluginDefineByTypeRequest, GetDsPluginDefineRequest};

/// 获取插件定义
pub(crate) async fn get_ui_plugins(id: i32) -> Result<UiPlugin> {
    let mut client = _ds_plugin_define_service_client().await?;
    let req = tonic::Request::new(GetDsPluginDefineRequest { id });
    let resp = client.get_ds_plugin_define(req).await?;
    let plugin = resp.into_inner();
    Ok(UiPlugin::from(plugin))
}

pub(crate) async fn get_ui_by_type(ui_type: &str) -> Result<Vec<UiPluginNoParams>> {
    let mut client = _ds_plugin_define_service_client().await?;
    let req = tonic::Request::new(GetDsPluginDefineByTypeRequest {
        ui_type: ui_type.to_string(),
    });
    let resp = client.get_ds_plugin_define_by_type(req).await?;
    let plugins = resp.into_inner();
    Ok(plugins.ds_plugin_defines.into_iter().map(UiPluginNoParams::from).collect())
}
