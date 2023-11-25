#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsEnvironment {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int64, tag = "2")]
    pub code: i64,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "6")]
    pub operator: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "7")]
    pub worker_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsEnvironmentsRequest {
    /// The maximum number of items to return.
    #[prost(uint64, tag = "1")]
    pub page_size: u64,
    #[prost(uint64, tag = "2")]
    pub page_num: u64,
    #[prost(string, optional, tag = "3")]
    pub search_val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsEnvironmentsResponse {
    #[prost(message, repeated, tag = "1")]
    pub total_list: ::prost::alloc::vec::Vec<DsEnvironmentPage>,
    #[prost(uint64, tag = "2")]
    pub current_page: u64,
    #[prost(uint64, tag = "3")]
    pub page_size: u64,
    #[prost(uint64, tag = "4")]
    pub start: u64,
    #[prost(uint64, tag = "5")]
    pub total: u64,
    #[prost(uint64, tag = "6")]
    pub total_page: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsEnvironmentPage {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int64, tag = "2")]
    pub code: i64,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "6")]
    pub operator: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "7")]
    pub worker_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsEnvironmentRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsEnvironmentRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub config: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, tag = "4")]
    pub operator: i32,
    #[prost(string, repeated, tag = "5")]
    pub worker_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsEnvironmentRequest {
    #[prost(int64, tag = "2")]
    pub code: i64,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub worker_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsEnvironmentRequest {
    /// The resource name of the DsEnvironment to be deleted.
    #[prost(int64, tag = "1")]
    pub code: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyDsEnvironmentRequest {
    #[prost(string, tag = "1")]
    pub environment_name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_environment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsEnvironmentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsEnvironmentServiceClient<tonic::transport::Channel> {
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
    impl<T> DsEnvironmentServiceClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> DsEnvironmentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
        {
            DsEnvironmentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_ds_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsEnvironmentsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsEnvironmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_environment.DsEnvironmentService/ListDsEnvironments");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_environment.DsEnvironmentService",
                "ListDsEnvironments",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::DsEnvironment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_environment.DsEnvironmentService/GetDsEnvironment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_environment.DsEnvironmentService",
                "GetDsEnvironment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::DsEnvironment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_environment.DsEnvironmentService/CreateDsEnvironment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_environment.DsEnvironmentService",
                "CreateDsEnvironment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::DsEnvironment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_environment.DsEnvironmentService/UpdateDsEnvironment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_environment.DsEnvironmentService",
                "UpdateDsEnvironment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_environment.DsEnvironmentService/DeleteDsEnvironment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_environment.DsEnvironmentService",
                "DeleteDsEnvironment",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_ds_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_environment.DsEnvironmentService/VerifyDsEnvironment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_environment.DsEnvironmentService",
                "VerifyDsEnvironment",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_environment_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsEnvironmentServiceServer.
    #[async_trait]
    pub trait DsEnvironmentService: Send + Sync + 'static {
        async fn list_ds_environments(
            &self,
            request: tonic::Request<super::ListDsEnvironmentsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsEnvironmentsResponse>, tonic::Status>;
        async fn get_ds_environment(
            &self,
            request: tonic::Request<super::GetDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::DsEnvironment>, tonic::Status>;
        async fn create_ds_environment(
            &self,
            request: tonic::Request<super::CreateDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::DsEnvironment>, tonic::Status>;
        async fn update_ds_environment(
            &self,
            request: tonic::Request<super::UpdateDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::DsEnvironment>, tonic::Status>;
        async fn delete_ds_environment(
            &self,
            request: tonic::Request<super::DeleteDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn verify_ds_environment(
            &self,
            request: tonic::Request<super::VerifyDsEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsEnvironmentServiceServer<T: DsEnvironmentService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsEnvironmentService> DsEnvironmentServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsEnvironmentServiceServer<T>
    where
        T: DsEnvironmentService,
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
                "/ds_environment.DsEnvironmentService/ListDsEnvironments" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsEnvironmentsSvc<T: DsEnvironmentService>(pub Arc<T>);
                    impl<T: DsEnvironmentService> tonic::server::UnaryService<super::ListDsEnvironmentsRequest>
                        for ListDsEnvironmentsSvc<T>
                    {
                        type Response = super::ListDsEnvironmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ListDsEnvironmentsRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_environments(request).await };
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
                        let method = ListDsEnvironmentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_environment.DsEnvironmentService/GetDsEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsEnvironmentSvc<T: DsEnvironmentService>(pub Arc<T>);
                    impl<T: DsEnvironmentService> tonic::server::UnaryService<super::GetDsEnvironmentRequest> for GetDsEnvironmentSvc<T> {
                        type Response = super::DsEnvironment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetDsEnvironmentRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_environment(request).await };
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
                        let method = GetDsEnvironmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_environment.DsEnvironmentService/CreateDsEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsEnvironmentSvc<T: DsEnvironmentService>(pub Arc<T>);
                    impl<T: DsEnvironmentService> tonic::server::UnaryService<super::CreateDsEnvironmentRequest>
                        for CreateDsEnvironmentSvc<T>
                    {
                        type Response = super::DsEnvironment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::CreateDsEnvironmentRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_ds_environment(request).await };
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
                        let method = CreateDsEnvironmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_environment.DsEnvironmentService/UpdateDsEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsEnvironmentSvc<T: DsEnvironmentService>(pub Arc<T>);
                    impl<T: DsEnvironmentService> tonic::server::UnaryService<super::UpdateDsEnvironmentRequest>
                        for UpdateDsEnvironmentSvc<T>
                    {
                        type Response = super::DsEnvironment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::UpdateDsEnvironmentRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ds_environment(request).await };
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
                        let method = UpdateDsEnvironmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_environment.DsEnvironmentService/DeleteDsEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsEnvironmentSvc<T: DsEnvironmentService>(pub Arc<T>);
                    impl<T: DsEnvironmentService> tonic::server::UnaryService<super::DeleteDsEnvironmentRequest>
                        for DeleteDsEnvironmentSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::DeleteDsEnvironmentRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_ds_environment(request).await };
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
                        let method = DeleteDsEnvironmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_environment.DsEnvironmentService/VerifyDsEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyDsEnvironmentSvc<T: DsEnvironmentService>(pub Arc<T>);
                    impl<T: DsEnvironmentService> tonic::server::UnaryService<super::VerifyDsEnvironmentRequest>
                        for VerifyDsEnvironmentSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::VerifyDsEnvironmentRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).verify_ds_environment(request).await };
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
                        let method = VerifyDsEnvironmentSvc(inner);
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
    impl<T: DsEnvironmentService> Clone for DsEnvironmentServiceServer<T> {
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
    impl<T: DsEnvironmentService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsEnvironmentService> tonic::server::NamedService for DsEnvironmentServiceServer<T> {
        const NAME: &'static str = "ds_environment.DsEnvironmentService";
    }
}
