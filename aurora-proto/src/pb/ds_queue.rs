#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsQueue {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub queue_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub queue: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
/// message DsQueue {
/// ! This should be defined elsewhere
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllDsQueuesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<DsQueue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsQueuesRequest {
    #[prost(uint64, tag = "1")]
    pub page_size: u64,
    #[prost(uint64, tag = "2")]
    pub page_num: u64,
    #[prost(string, optional, tag = "3")]
    pub search_val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsQueuesResponse {
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub total_list: ::prost::alloc::vec::Vec<DsQueue>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsQueueRequest {
    #[prost(string, tag = "1")]
    pub queue: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub queue_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyQueueRequest {
    #[prost(string, tag = "1")]
    pub queue: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub queue_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyQueue {
    #[prost(message, optional, tag = "1")]
    pub ds_queue: ::core::option::Option<DsQueue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsQueueRequest {
    #[prost(string, tag = "1")]
    pub queue: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub queue_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsQueueRequest {
    /// The DsQueue resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_queue: ::core::option::Option<DsQueue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsQueueRequest {
    /// The resource name of the DsQueue to be deleted.
    #[prost(int32, tag = "1")]
    pub id: i32,
}
/// Generated client implementations.
pub mod ds_queue_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsQueueServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsQueueServiceClient<tonic::transport::Channel> {
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
    impl<T> DsQueueServiceClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> DsQueueServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
        {
            DsQueueServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_ds_queues(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsQueuesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsQueuesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/ListDsQueues");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "ListDsQueues"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/GetDsQueue");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "GetDsQueue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_ds_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::VerifyQueue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/VerifyDsQueue");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "VerifyDsQueue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/CreateDsQueue");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "CreateDsQueue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/UpdateDsQueue");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "UpdateDsQueue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/DeleteDsQueue");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "DeleteDsQueue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_ds_queues(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::AllDsQueuesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_queue.DsQueueService/AllDsQueues");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_queue.DsQueueService", "AllDsQueues"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_queue_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsQueueServiceServer.
    #[async_trait]
    pub trait DsQueueService: Send + Sync + 'static {
        async fn list_ds_queues(
            &self,
            request: tonic::Request<super::ListDsQueuesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsQueuesResponse>, tonic::Status>;
        async fn get_ds_queue(
            &self,
            request: tonic::Request<super::GetDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueue>, tonic::Status>;
        async fn verify_ds_queue(
            &self,
            request: tonic::Request<super::VerifyQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::VerifyQueue>, tonic::Status>;
        async fn create_ds_queue(
            &self,
            request: tonic::Request<super::CreateDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueue>, tonic::Status>;
        async fn update_ds_queue(
            &self,
            request: tonic::Request<super::UpdateDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueue>, tonic::Status>;
        async fn delete_ds_queue(
            &self,
            request: tonic::Request<super::DeleteDsQueueRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn all_ds_queues(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::AllDsQueuesResponse>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsQueueServiceServer<T: DsQueueService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsQueueService> DsQueueServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsQueueServiceServer<T>
    where
        T: DsQueueService,
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
                "/ds_queue.DsQueueService/ListDsQueues" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsQueuesSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<super::ListDsQueuesRequest> for ListDsQueuesSvc<T> {
                        type Response = super::ListDsQueuesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ListDsQueuesRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_queues(request).await };
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
                        let method = ListDsQueuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_queue.DsQueueService/GetDsQueue" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsQueueSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<super::GetDsQueueRequest> for GetDsQueueSvc<T> {
                        type Response = super::DsQueue;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetDsQueueRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_queue(request).await };
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
                        let method = GetDsQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_queue.DsQueueService/VerifyDsQueue" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyDsQueueSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<super::VerifyQueueRequest> for VerifyDsQueueSvc<T> {
                        type Response = super::VerifyQueue;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::VerifyQueueRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).verify_ds_queue(request).await };
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
                        let method = VerifyDsQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_queue.DsQueueService/CreateDsQueue" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsQueueSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<super::CreateDsQueueRequest> for CreateDsQueueSvc<T> {
                        type Response = super::DsQueue;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::CreateDsQueueRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_ds_queue(request).await };
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
                        let method = CreateDsQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_queue.DsQueueService/UpdateDsQueue" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsQueueSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<super::UpdateDsQueueRequest> for UpdateDsQueueSvc<T> {
                        type Response = super::DsQueue;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::UpdateDsQueueRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ds_queue(request).await };
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
                        let method = UpdateDsQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_queue.DsQueueService/DeleteDsQueue" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsQueueSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<super::DeleteDsQueueRequest> for DeleteDsQueueSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::DeleteDsQueueRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_ds_queue(request).await };
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
                        let method = DeleteDsQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_queue.DsQueueService/AllDsQueues" => {
                    #[allow(non_camel_case_types)]
                    struct AllDsQueuesSvc<T: DsQueueService>(pub Arc<T>);
                    impl<T: DsQueueService> tonic::server::UnaryService<()> for AllDsQueuesSvc<T> {
                        type Response = super::AllDsQueuesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_ds_queues(request).await };
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
                        let method = AllDsQueuesSvc(inner);
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
    impl<T: DsQueueService> Clone for DsQueueServiceServer<T> {
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
    impl<T: DsQueueService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsQueueService> tonic::server::NamedService for DsQueueServiceServer<T> {
        const NAME: &'static str = "ds_queue.DsQueueService";
    }
}
