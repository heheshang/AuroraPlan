#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsAuditLog {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub user_id: i32,
    #[prost(int32, tag = "3")]
    pub resource_ds_audit_log: i32,
    #[prost(int32, tag = "4")]
    pub operation: i32,
    #[prost(string, optional, tag = "5")]
    pub time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, tag = "6")]
    pub resource_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsAuditLogsRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsAuditLogsResponse {
    /// The field name should match the noun "DsAuditLog" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_audit_logs: ::prost::alloc::vec::Vec<DsAuditLog>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsAuditLogRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsAuditLogRequest {
    /// The parent resource name where the DsAuditLog is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsAuditLog id to use for this DsAuditLog.
    #[prost(string, tag = "2")]
    pub ds_audit_log_id: ::prost::alloc::string::String,
    /// The DsAuditLog resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_audit_log: ::core::option::Option<DsAuditLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsAuditLogRequest {
    /// The DsAuditLog resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_audit_log: ::core::option::Option<DsAuditLog>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsAuditLogRequest {
    /// The resource name of the DsAuditLog to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_audit_log_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsAuditLogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsAuditLogServiceClient<tonic::transport::Channel> {
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
    impl<T> DsAuditLogServiceClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> DsAuditLogServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
        {
            DsAuditLogServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_ds_audit_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsAuditLogsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsAuditLogsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_audit_log.DsAuditLogService/ListDsAuditLogs");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_audit_log.DsAuditLogService", "ListDsAuditLogs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_audit_log(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAuditLog>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_audit_log.DsAuditLogService/GetDsAuditLog");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_audit_log.DsAuditLogService", "GetDsAuditLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_audit_log(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAuditLog>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_audit_log.DsAuditLogService/CreateDsAuditLog");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_audit_log.DsAuditLogService", "CreateDsAuditLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_audit_log(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAuditLog>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_audit_log.DsAuditLogService/UpdateDsAuditLog");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_audit_log.DsAuditLogService", "UpdateDsAuditLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_audit_log(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ds_audit_log.DsAuditLogService/DeleteDsAuditLog");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ds_audit_log.DsAuditLogService", "DeleteDsAuditLog"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_audit_log_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsAuditLogServiceServer.
    #[async_trait]
    pub trait DsAuditLogService: Send + Sync + 'static {
        async fn list_ds_audit_logs(
            &self,
            request: tonic::Request<super::ListDsAuditLogsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsAuditLogsResponse>, tonic::Status>;
        async fn get_ds_audit_log(
            &self,
            request: tonic::Request<super::GetDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAuditLog>, tonic::Status>;
        async fn create_ds_audit_log(
            &self,
            request: tonic::Request<super::CreateDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAuditLog>, tonic::Status>;
        async fn update_ds_audit_log(
            &self,
            request: tonic::Request<super::UpdateDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<super::DsAuditLog>, tonic::Status>;
        async fn delete_ds_audit_log(
            &self,
            request: tonic::Request<super::DeleteDsAuditLogRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsAuditLogServiceServer<T: DsAuditLogService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsAuditLogService> DsAuditLogServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsAuditLogServiceServer<T>
    where
        T: DsAuditLogService,
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
                "/ds_audit_log.DsAuditLogService/ListDsAuditLogs" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsAuditLogsSvc<T: DsAuditLogService>(pub Arc<T>);
                    impl<T: DsAuditLogService> tonic::server::UnaryService<super::ListDsAuditLogsRequest> for ListDsAuditLogsSvc<T> {
                        type Response = super::ListDsAuditLogsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ListDsAuditLogsRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_audit_logs(request).await };
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
                        let method = ListDsAuditLogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_audit_log.DsAuditLogService/GetDsAuditLog" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsAuditLogSvc<T: DsAuditLogService>(pub Arc<T>);
                    impl<T: DsAuditLogService> tonic::server::UnaryService<super::GetDsAuditLogRequest> for GetDsAuditLogSvc<T> {
                        type Response = super::DsAuditLog;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetDsAuditLogRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_audit_log(request).await };
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
                        let method = GetDsAuditLogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_audit_log.DsAuditLogService/CreateDsAuditLog" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsAuditLogSvc<T: DsAuditLogService>(pub Arc<T>);
                    impl<T: DsAuditLogService> tonic::server::UnaryService<super::CreateDsAuditLogRequest> for CreateDsAuditLogSvc<T> {
                        type Response = super::DsAuditLog;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::CreateDsAuditLogRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_ds_audit_log(request).await };
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
                        let method = CreateDsAuditLogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_audit_log.DsAuditLogService/UpdateDsAuditLog" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsAuditLogSvc<T: DsAuditLogService>(pub Arc<T>);
                    impl<T: DsAuditLogService> tonic::server::UnaryService<super::UpdateDsAuditLogRequest> for UpdateDsAuditLogSvc<T> {
                        type Response = super::DsAuditLog;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::UpdateDsAuditLogRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ds_audit_log(request).await };
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
                        let method = UpdateDsAuditLogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_audit_log.DsAuditLogService/DeleteDsAuditLog" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsAuditLogSvc<T: DsAuditLogService>(pub Arc<T>);
                    impl<T: DsAuditLogService> tonic::server::UnaryService<super::DeleteDsAuditLogRequest> for DeleteDsAuditLogSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::DeleteDsAuditLogRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_ds_audit_log(request).await };
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
                        let method = DeleteDsAuditLogSvc(inner);
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
    impl<T: DsAuditLogService> Clone for DsAuditLogServiceServer<T> {
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
    impl<T: DsAuditLogService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsAuditLogService> tonic::server::NamedService for DsAuditLogServiceServer<T> {
        const NAME: &'static str = "ds_audit_log.DsAuditLogService";
    }
}
