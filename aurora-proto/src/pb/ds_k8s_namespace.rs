#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsK8sNamespace {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, optional, tag = "2")]
    pub limits_memory: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub online_job_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub user_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub pod_replicas: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub pod_request_cpu: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub pod_request_memory: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub limits_cpu: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub k8s: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsK8sNamespacesRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsK8sNamespacesResponse {
    /// The field name should match the noun "DsK8sNamespace" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_k8s_namespaces: ::prost::alloc::vec::Vec<DsK8sNamespace>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsK8sNamespaceRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsK8sNamespaceRequest {
    /// The parent resource name where the DsK8sNamespace is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsK8sNamespace id to use for this DsK8sNamespace.
    #[prost(string, tag = "2")]
    pub ds_k8s_namespace_id: ::prost::alloc::string::String,
    /// The DsK8sNamespace resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_k8s_namespace: ::core::option::Option<DsK8sNamespace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsK8sNamespaceRequest {
    /// The DsK8sNamespace resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_k8s_namespace: ::core::option::Option<DsK8sNamespace>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsK8sNamespaceRequest {
    /// The resource name of the DsK8sNamespace to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_k8s_namespace_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsK8sNamespaceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsK8sNamespaceServiceClient<tonic::transport::Channel> {
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
    impl<T> DsK8sNamespaceServiceClient<T>
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
        ) -> DsK8sNamespaceServiceClient<InterceptedService<T, F>>
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
            DsK8sNamespaceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_ds_k8s_namespaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsK8sNamespacesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsK8sNamespacesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_k8s_namespace.DsK8sNamespaceService/ListDsK8sNamespaces",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_k8s_namespace.DsK8sNamespaceService",
                "ListDsK8sNamespaces",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_k8s_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsK8sNamespace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_k8s_namespace.DsK8sNamespaceService/GetDsK8sNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_k8s_namespace.DsK8sNamespaceService",
                "GetDsK8sNamespace",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_k8s_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsK8sNamespace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_k8s_namespace.DsK8sNamespaceService/CreateDsK8sNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_k8s_namespace.DsK8sNamespaceService",
                "CreateDsK8sNamespace",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_k8s_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsK8sNamespace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_k8s_namespace.DsK8sNamespaceService/UpdateDsK8sNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_k8s_namespace.DsK8sNamespaceService",
                "UpdateDsK8sNamespace",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_k8s_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_k8s_namespace.DsK8sNamespaceService/DeleteDsK8sNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_k8s_namespace.DsK8sNamespaceService",
                "DeleteDsK8sNamespace",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_k8s_namespace_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsK8sNamespaceServiceServer.
    #[async_trait]
    pub trait DsK8sNamespaceService: Send + Sync + 'static {
        async fn list_ds_k8s_namespaces(
            &self,
            request: tonic::Request<super::ListDsK8sNamespacesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsK8sNamespacesResponse>, tonic::Status>;
        async fn get_ds_k8s_namespace(
            &self,
            request: tonic::Request<super::GetDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsK8sNamespace>, tonic::Status>;
        async fn create_ds_k8s_namespace(
            &self,
            request: tonic::Request<super::CreateDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsK8sNamespace>, tonic::Status>;
        async fn update_ds_k8s_namespace(
            &self,
            request: tonic::Request<super::UpdateDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::DsK8sNamespace>, tonic::Status>;
        async fn delete_ds_k8s_namespace(
            &self,
            request: tonic::Request<super::DeleteDsK8sNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsK8sNamespaceServiceServer<T: DsK8sNamespaceService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsK8sNamespaceService> DsK8sNamespaceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsK8sNamespaceServiceServer<T>
    where
        T: DsK8sNamespaceService,
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
                "/ds_k8s_namespace.DsK8sNamespaceService/ListDsK8sNamespaces" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsK8sNamespacesSvc<T: DsK8sNamespaceService>(pub Arc<T>);
                    impl<T: DsK8sNamespaceService>
                        tonic::server::UnaryService<super::ListDsK8sNamespacesRequest>
                        for ListDsK8sNamespacesSvc<T>
                    {
                        type Response = super::ListDsK8sNamespacesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsK8sNamespacesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_k8s_namespaces(request).await };
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
                        let method = ListDsK8sNamespacesSvc(inner);
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
                "/ds_k8s_namespace.DsK8sNamespaceService/GetDsK8sNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsK8sNamespaceSvc<T: DsK8sNamespaceService>(pub Arc<T>);
                    impl<T: DsK8sNamespaceService>
                        tonic::server::UnaryService<super::GetDsK8sNamespaceRequest>
                        for GetDsK8sNamespaceSvc<T>
                    {
                        type Response = super::DsK8sNamespace;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsK8sNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_k8s_namespace(request).await };
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
                        let method = GetDsK8sNamespaceSvc(inner);
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
                "/ds_k8s_namespace.DsK8sNamespaceService/CreateDsK8sNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsK8sNamespaceSvc<T: DsK8sNamespaceService>(pub Arc<T>);
                    impl<T: DsK8sNamespaceService>
                        tonic::server::UnaryService<super::CreateDsK8sNamespaceRequest>
                        for CreateDsK8sNamespaceSvc<T>
                    {
                        type Response = super::DsK8sNamespace;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsK8sNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_ds_k8s_namespace(request).await };
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
                        let method = CreateDsK8sNamespaceSvc(inner);
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
                "/ds_k8s_namespace.DsK8sNamespaceService/UpdateDsK8sNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsK8sNamespaceSvc<T: DsK8sNamespaceService>(pub Arc<T>);
                    impl<T: DsK8sNamespaceService>
                        tonic::server::UnaryService<super::UpdateDsK8sNamespaceRequest>
                        for UpdateDsK8sNamespaceSvc<T>
                    {
                        type Response = super::DsK8sNamespace;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsK8sNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_ds_k8s_namespace(request).await };
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
                        let method = UpdateDsK8sNamespaceSvc(inner);
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
                "/ds_k8s_namespace.DsK8sNamespaceService/DeleteDsK8sNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsK8sNamespaceSvc<T: DsK8sNamespaceService>(pub Arc<T>);
                    impl<T: DsK8sNamespaceService>
                        tonic::server::UnaryService<super::DeleteDsK8sNamespaceRequest>
                        for DeleteDsK8sNamespaceSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsK8sNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_ds_k8s_namespace(request).await };
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
                        let method = DeleteDsK8sNamespaceSvc(inner);
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
    impl<T: DsK8sNamespaceService> Clone for DsK8sNamespaceServiceServer<T> {
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
    impl<T: DsK8sNamespaceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsK8sNamespaceService> tonic::server::NamedService for DsK8sNamespaceServiceServer<T> {
        const NAME: &'static str = "ds_k8s_namespace.DsK8sNamespaceService";
    }
}
