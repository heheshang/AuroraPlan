#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsAlertGroup {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub alert_instance_ids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub create_user_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub group_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAlertGroupRequest {
    #[prost(string, tag = "1")]
    pub group_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsAlertGroupsRequest {
    #[prost(int64, tag = "1")]
    pub page_size: i64,
    #[prost(int64, tag = "2")]
    pub page_num: i64,
    #[prost(string, optional, tag = "3")]
    pub search_val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsAlertGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub total_list: ::prost::alloc::vec::Vec<DsAlertGroup>,
    #[prost(int64, tag = "2")]
    pub current_page: i64,
    #[prost(int64, tag = "3")]
    pub page_size: i64,
    #[prost(int64, tag = "4")]
    pub start: i64,
    #[prost(int64, tag = "5")]
    pub total: i64,
    #[prost(int64, tag = "6")]
    pub total_page: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsAlertGroupRequest {
    /// The field will contain name of the resource requested.
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsAlertGroupRequest {
    #[prost(string, optional, tag = "2")]
    pub alert_instance_ids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub create_user_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub group_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsAlertGroupRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub alert_instance_ids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub create_user_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub group_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsAlertGroupRequest {
    /// The resource name of the DsAlertGroup to be deleted.
    #[prost(int32, tag = "1")]
    pub id: i32,
}
/// Generated client implementations.
pub mod ds_alert_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsAlertGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsAlertGroupServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DsAlertGroupServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> DsAlertGroupServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
        {
            DsAlertGroupServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn list_ds_alert_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsAlertGroupsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsAlertGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_alertgroup.DsAlertGroupService/ListDsAlertGroups");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_alertgroup.DsAlertGroupService",
                "ListDsAlertGroups",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_alert_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAlertGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_alertgroup.DsAlertGroupService/GetDsAlertGroup");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_alertgroup.DsAlertGroupService", "GetDsAlertGroup"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_alert_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAlertGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_alertgroup.DsAlertGroupService/CreateDsAlertGroup");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_alertgroup.DsAlertGroupService",
                "CreateDsAlertGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_alert_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAlertGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_alertgroup.DsAlertGroupService/UpdateDsAlertGroup");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_alertgroup.DsAlertGroupService",
                "UpdateDsAlertGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_alert_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_alertgroup.DsAlertGroupService/DeleteDsAlertGroup");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_alertgroup.DsAlertGroupService",
                "DeleteDsAlertGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_alert_group(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_alertgroup.DsAlertGroupService/VerifyAlertGroup");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_alertgroup.DsAlertGroupService", "VerifyAlertGroup"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_alert_group_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsAlertGroupServiceServer.
    #[async_trait]
    pub trait DsAlertGroupService: Send + Sync + 'static {
        async fn list_ds_alert_groups(
            &self,
            request: tonic::Request<super::ListDsAlertGroupsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsAlertGroupsResponse>, tonic::Status>;
        async fn get_ds_alert_group(
            &self,
            request: tonic::Request<super::GetDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAlertGroup>, tonic::Status>;
        async fn create_ds_alert_group(
            &self,
            request: tonic::Request<super::CreateDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAlertGroup>, tonic::Status>;
        async fn update_ds_alert_group(
            &self,
            request: tonic::Request<super::UpdateDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAlertGroup>, tonic::Status>;
        async fn delete_ds_alert_group(
            &self,
            request: tonic::Request<super::DeleteDsAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn verify_alert_group(
            &self,
            request: tonic::Request<super::VerifyAlertGroupRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsAlertGroupServiceServer<T: DsAlertGroupService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsAlertGroupService> DsAlertGroupServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsAlertGroupServiceServer<T>
    where
        T: DsAlertGroupService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ds_alertgroup.DsAlertGroupService/ListDsAlertGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsAlertGroupsSvc<T: DsAlertGroupService>(pub Arc<T>);
                    impl<T: DsAlertGroupService> tonic::server::UnaryService<super::ListDsAlertGroupsRequest> for ListDsAlertGroupsSvc<T> {
                        type Response = super::ListDsAlertGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ListDsAlertGroupsRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_alert_groups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDsAlertGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_alertgroup.DsAlertGroupService/GetDsAlertGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsAlertGroupSvc<T: DsAlertGroupService>(pub Arc<T>);
                    impl<T: DsAlertGroupService> tonic::server::UnaryService<super::GetDsAlertGroupRequest> for GetDsAlertGroupSvc<T> {
                        type Response = super::DsAlertGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetDsAlertGroupRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_alert_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDsAlertGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_alertgroup.DsAlertGroupService/CreateDsAlertGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsAlertGroupSvc<T: DsAlertGroupService>(pub Arc<T>);
                    impl<T: DsAlertGroupService> tonic::server::UnaryService<super::CreateDsAlertGroupRequest>
                        for CreateDsAlertGroupSvc<T>
                    {
                        type Response = super::DsAlertGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::CreateDsAlertGroupRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_ds_alert_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDsAlertGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_alertgroup.DsAlertGroupService/UpdateDsAlertGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsAlertGroupSvc<T: DsAlertGroupService>(pub Arc<T>);
                    impl<T: DsAlertGroupService> tonic::server::UnaryService<super::UpdateDsAlertGroupRequest>
                        for UpdateDsAlertGroupSvc<T>
                    {
                        type Response = super::DsAlertGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::UpdateDsAlertGroupRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ds_alert_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateDsAlertGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_alertgroup.DsAlertGroupService/DeleteDsAlertGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsAlertGroupSvc<T: DsAlertGroupService>(pub Arc<T>);
                    impl<T: DsAlertGroupService> tonic::server::UnaryService<super::DeleteDsAlertGroupRequest>
                        for DeleteDsAlertGroupSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::DeleteDsAlertGroupRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_ds_alert_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDsAlertGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_alertgroup.DsAlertGroupService/VerifyAlertGroup" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyAlertGroupSvc<T: DsAlertGroupService>(pub Arc<T>);
                    impl<T: DsAlertGroupService> tonic::server::UnaryService<super::VerifyAlertGroupRequest> for VerifyAlertGroupSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::VerifyAlertGroupRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).verify_alert_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyAlertGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DsAlertGroupService> Clone for DsAlertGroupServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DsAlertGroupService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsAlertGroupService> tonic::server::NamedService for DsAlertGroupServiceServer<T> {
        const NAME: &'static str = "ds_alertgroup.DsAlertGroupService";
    }
}
