#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzSimpleTrigger {
    #[prost(string, tag = "1")]
    pub sched_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub trigger_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub trigger_group: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub repeat_count: i64,
    #[prost(int64, tag = "5")]
    pub repeat_interval: i64,
    #[prost(int64, tag = "6")]
    pub times_triggered: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzSimpleTriggersRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzSimpleTriggersResponse {
    /// The field name should match the noun "QrtzSimpleTrigger" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_simple_triggers: ::prost::alloc::vec::Vec<QrtzSimpleTrigger>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzSimpleTriggerRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzSimpleTriggerRequest {
    /// The parent resource name where the QrtzSimpleTrigger is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzSimpleTrigger id to use for this QrtzSimpleTrigger.
    #[prost(string, tag = "2")]
    pub qrtz_simple_trigger_id: ::prost::alloc::string::String,
    /// The QrtzSimpleTrigger resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_simple_trigger: ::core::option::Option<QrtzSimpleTrigger>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzSimpleTriggerRequest {
    /// The QrtzSimpleTrigger resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_simple_trigger: ::core::option::Option<QrtzSimpleTrigger>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzSimpleTriggerRequest {
    /// The resource name of the QrtzSimpleTrigger to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_simple_trigger_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzSimpleTriggerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzSimpleTriggerServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzSimpleTriggerServiceClient<T>
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
        ) -> QrtzSimpleTriggerServiceClient<InterceptedService<T, F>>
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
            QrtzSimpleTriggerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_qrtz_simple_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzSimpleTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzSimpleTriggersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/ListQrtzSimpleTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_simple_triggers.QrtzSimpleTriggerService",
                "ListQrtzSimpleTriggers",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_qrtz_simple_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzSimpleTrigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/GetQrtzSimpleTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_simple_triggers.QrtzSimpleTriggerService",
                "GetQrtzSimpleTrigger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_qrtz_simple_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzSimpleTrigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/CreateQrtzSimpleTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_simple_triggers.QrtzSimpleTriggerService",
                "CreateQrtzSimpleTrigger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_qrtz_simple_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzSimpleTrigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/UpdateQrtzSimpleTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_simple_triggers.QrtzSimpleTriggerService",
                "UpdateQrtzSimpleTrigger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_qrtz_simple_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/DeleteQrtzSimpleTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_simple_triggers.QrtzSimpleTriggerService",
                "DeleteQrtzSimpleTrigger",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_simple_trigger_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzSimpleTriggerServiceServer.
    #[async_trait]
    pub trait QrtzSimpleTriggerService: Send + Sync + 'static {
        async fn list_qrtz_simple_triggers(
            &self,
            request: tonic::Request<super::ListQrtzSimpleTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzSimpleTriggersResponse>,
            tonic::Status,
        >;
        async fn get_qrtz_simple_trigger(
            &self,
            request: tonic::Request<super::GetQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzSimpleTrigger>, tonic::Status>;
        async fn create_qrtz_simple_trigger(
            &self,
            request: tonic::Request<super::CreateQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzSimpleTrigger>, tonic::Status>;
        async fn update_qrtz_simple_trigger(
            &self,
            request: tonic::Request<super::UpdateQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzSimpleTrigger>, tonic::Status>;
        async fn delete_qrtz_simple_trigger(
            &self,
            request: tonic::Request<super::DeleteQrtzSimpleTriggerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzSimpleTriggerServiceServer<T: QrtzSimpleTriggerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzSimpleTriggerService> QrtzSimpleTriggerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QrtzSimpleTriggerServiceServer<T>
    where
        T: QrtzSimpleTriggerService,
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
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/ListQrtzSimpleTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzSimpleTriggersSvc<T: QrtzSimpleTriggerService>(pub Arc<T>);
                    impl<T: QrtzSimpleTriggerService>
                        tonic::server::UnaryService<super::ListQrtzSimpleTriggersRequest>
                        for ListQrtzSimpleTriggersSvc<T>
                    {
                        type Response = super::ListQrtzSimpleTriggersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQrtzSimpleTriggersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).list_qrtz_simple_triggers(request).await };
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
                        let method = ListQrtzSimpleTriggersSvc(inner);
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
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/GetQrtzSimpleTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzSimpleTriggerSvc<T: QrtzSimpleTriggerService>(pub Arc<T>);
                    impl<T: QrtzSimpleTriggerService>
                        tonic::server::UnaryService<super::GetQrtzSimpleTriggerRequest>
                        for GetQrtzSimpleTriggerSvc<T>
                    {
                        type Response = super::QrtzSimpleTrigger;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzSimpleTriggerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).get_qrtz_simple_trigger(request).await };
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
                        let method = GetQrtzSimpleTriggerSvc(inner);
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
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/CreateQrtzSimpleTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzSimpleTriggerSvc<T: QrtzSimpleTriggerService>(pub Arc<T>);
                    impl<T: QrtzSimpleTriggerService>
                        tonic::server::UnaryService<super::CreateQrtzSimpleTriggerRequest>
                        for CreateQrtzSimpleTriggerSvc<T>
                    {
                        type Response = super::QrtzSimpleTrigger;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateQrtzSimpleTriggerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_qrtz_simple_trigger(request).await };
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
                        let method = CreateQrtzSimpleTriggerSvc(inner);
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
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/UpdateQrtzSimpleTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzSimpleTriggerSvc<T: QrtzSimpleTriggerService>(pub Arc<T>);
                    impl<T: QrtzSimpleTriggerService>
                        tonic::server::UnaryService<super::UpdateQrtzSimpleTriggerRequest>
                        for UpdateQrtzSimpleTriggerSvc<T>
                    {
                        type Response = super::QrtzSimpleTrigger;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateQrtzSimpleTriggerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_qrtz_simple_trigger(request).await };
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
                        let method = UpdateQrtzSimpleTriggerSvc(inner);
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
                "/qrtz_simple_triggers.QrtzSimpleTriggerService/DeleteQrtzSimpleTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzSimpleTriggerSvc<T: QrtzSimpleTriggerService>(pub Arc<T>);
                    impl<T: QrtzSimpleTriggerService>
                        tonic::server::UnaryService<super::DeleteQrtzSimpleTriggerRequest>
                        for DeleteQrtzSimpleTriggerSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteQrtzSimpleTriggerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_qrtz_simple_trigger(request).await };
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
                        let method = DeleteQrtzSimpleTriggerSvc(inner);
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
    impl<T: QrtzSimpleTriggerService> Clone for QrtzSimpleTriggerServiceServer<T> {
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
    impl<T: QrtzSimpleTriggerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzSimpleTriggerService> tonic::server::NamedService
        for QrtzSimpleTriggerServiceServer<T>
    {
        const NAME: &'static str = "qrtz_simple_triggers.QrtzSimpleTriggerService";
    }
}
