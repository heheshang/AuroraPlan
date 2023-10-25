#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsTaskInstance {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub task_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "4")]
    pub task_code: i64,
    #[prost(int32, optional, tag = "5")]
    pub task_definition_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub process_instance_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub state: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "8")]
    pub submit_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub start_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub end_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub execute_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub log_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "14")]
    pub alert_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub retry_times: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "16")]
    pub pid: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "17")]
    pub app_link: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub task_params: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "19")]
    pub flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "20")]
    pub retry_interval: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "21")]
    pub max_retry_times: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "22")]
    pub task_instance_priority: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "23")]
    pub worker_group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "24")]
    pub environment_code: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "25")]
    pub environment_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "26")]
    pub executor_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "27")]
    pub first_submit_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "28")]
    pub delay_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "29")]
    pub task_group_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "30")]
    pub var_pool: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "31")]
    pub dry_run: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsTaskInstancesRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsTaskInstancesResponse {
    /// The field name should match the noun "DsTaskInstance" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_task_instances: ::prost::alloc::vec::Vec<DsTaskInstance>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsTaskInstanceRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsTaskInstanceRequest {
    /// The parent resource name where the DsTaskInstance is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsTaskInstance id to use for this DsTaskInstance.
    #[prost(string, tag = "2")]
    pub ds_task_instance_id: ::prost::alloc::string::String,
    /// The DsTaskInstance resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_task_instance: ::core::option::Option<DsTaskInstance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsTaskInstanceRequest {
    /// The DsTaskInstance resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_task_instance: ::core::option::Option<DsTaskInstance>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsTaskInstanceRequest {
    /// The resource name of the DsTaskInstance to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_task_instance_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsTaskInstanceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsTaskInstanceServiceClient<tonic::transport::Channel> {
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
    impl<T> DsTaskInstanceServiceClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DsTaskInstanceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DsTaskInstanceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_ds_task_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsTaskInstancesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsTaskInstancesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_instance.DsTaskInstanceService/ListDsTaskInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_instance.DsTaskInstanceService",
                "ListDsTaskInstances",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_task_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskInstance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_instance.DsTaskInstanceService/GetDsTaskInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_instance.DsTaskInstanceService",
                "GetDsTaskInstance",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_task_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskInstance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_instance.DsTaskInstanceService/CreateDsTaskInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_instance.DsTaskInstanceService",
                "CreateDsTaskInstance",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_task_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskInstance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_instance.DsTaskInstanceService/UpdateDsTaskInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_instance.DsTaskInstanceService",
                "UpdateDsTaskInstance",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_task_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_instance.DsTaskInstanceService/DeleteDsTaskInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_instance.DsTaskInstanceService",
                "DeleteDsTaskInstance",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_task_instance_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsTaskInstanceServiceServer.
    #[async_trait]
    pub trait DsTaskInstanceService: Send + Sync + 'static {
        async fn list_ds_task_instances(
            &self,
            request: tonic::Request<super::ListDsTaskInstancesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsTaskInstancesResponse>, tonic::Status>;
        async fn get_ds_task_instance(
            &self,
            request: tonic::Request<super::GetDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskInstance>, tonic::Status>;
        async fn create_ds_task_instance(
            &self,
            request: tonic::Request<super::CreateDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskInstance>, tonic::Status>;
        async fn update_ds_task_instance(
            &self,
            request: tonic::Request<super::UpdateDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskInstance>, tonic::Status>;
        async fn delete_ds_task_instance(
            &self,
            request: tonic::Request<super::DeleteDsTaskInstanceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsTaskInstanceServiceServer<T: DsTaskInstanceService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsTaskInstanceService> DsTaskInstanceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsTaskInstanceServiceServer<T>
    where
        T: DsTaskInstanceService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ds_task_instance.DsTaskInstanceService/ListDsTaskInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsTaskInstancesSvc<T: DsTaskInstanceService>(pub Arc<T>);
                    impl<T: DsTaskInstanceService>
                        tonic::server::UnaryService<super::ListDsTaskInstancesRequest>
                        for ListDsTaskInstancesSvc<T>
                    {
                        type Response = super::ListDsTaskInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsTaskInstancesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_task_instances(request).await };
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
                        let method = ListDsTaskInstancesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_task_instance.DsTaskInstanceService/GetDsTaskInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsTaskInstanceSvc<T: DsTaskInstanceService>(pub Arc<T>);
                    impl<T: DsTaskInstanceService>
                        tonic::server::UnaryService<super::GetDsTaskInstanceRequest>
                        for GetDsTaskInstanceSvc<T>
                    {
                        type Response = super::DsTaskInstance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsTaskInstanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_task_instance(request).await };
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
                        let method = GetDsTaskInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_task_instance.DsTaskInstanceService/CreateDsTaskInstance" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsTaskInstanceSvc<T: DsTaskInstanceService>(pub Arc<T>);
                    impl<T: DsTaskInstanceService>
                        tonic::server::UnaryService<super::CreateDsTaskInstanceRequest>
                        for CreateDsTaskInstanceSvc<T>
                    {
                        type Response = super::DsTaskInstance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsTaskInstanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_ds_task_instance(request).await };
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
                        let method = CreateDsTaskInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_task_instance.DsTaskInstanceService/UpdateDsTaskInstance" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsTaskInstanceSvc<T: DsTaskInstanceService>(pub Arc<T>);
                    impl<T: DsTaskInstanceService>
                        tonic::server::UnaryService<super::UpdateDsTaskInstanceRequest>
                        for UpdateDsTaskInstanceSvc<T>
                    {
                        type Response = super::DsTaskInstance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsTaskInstanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_ds_task_instance(request).await };
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
                        let method = UpdateDsTaskInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_task_instance.DsTaskInstanceService/DeleteDsTaskInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsTaskInstanceSvc<T: DsTaskInstanceService>(pub Arc<T>);
                    impl<T: DsTaskInstanceService>
                        tonic::server::UnaryService<super::DeleteDsTaskInstanceRequest>
                        for DeleteDsTaskInstanceSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsTaskInstanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_ds_task_instance(request).await };
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
                        let method = DeleteDsTaskInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
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
    impl<T: DsTaskInstanceService> Clone for DsTaskInstanceServiceServer<T> {
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
    impl<T: DsTaskInstanceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsTaskInstanceService> tonic::server::NamedService for DsTaskInstanceServiceServer<T> {
        const NAME: &'static str = "ds_task_instance.DsTaskInstanceService";
    }
}
