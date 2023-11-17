#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzJobDetails {
    #[prost(string, tag = "1")]
    pub sched_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub job_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub job_group: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub job_class_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub is_durable: bool,
    #[prost(bool, tag = "7")]
    pub is_nonconcurrent: bool,
    #[prost(bool, tag = "8")]
    pub is_update_data: bool,
    #[prost(bool, tag = "9")]
    pub requests_recovery: bool,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub job_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzJobDetailssRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzJobDetailssResponse {
    /// The field name should match the noun "QrtzJobDetails" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_job_detailss: ::prost::alloc::vec::Vec<QrtzJobDetails>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzJobDetailsRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzJobDetailsRequest {
    /// The parent resource name where the QrtzJobDetails is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzJobDetails id to use for this QrtzJobDetails.
    #[prost(string, tag = "2")]
    pub qrtz_job_details_id: ::prost::alloc::string::String,
    /// The QrtzJobDetails resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_job_details: ::core::option::Option<QrtzJobDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzJobDetailsRequest {
    /// The QrtzJobDetails resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_job_details: ::core::option::Option<QrtzJobDetails>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzJobDetailsRequest {
    /// The resource name of the QrtzJobDetails to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_job_details_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzJobDetailsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzJobDetailsServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzJobDetailsServiceClient<T>
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
        pub fn with_origin(
            inner: T,
            origin: Uri,
        ) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QrtzJobDetailsServiceClient<InterceptedService<T, F>>
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
            QrtzJobDetailsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(
            mut self,
            limit: usize,
        ) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(
            mut self,
            limit: usize,
        ) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn list_qrtz_job_detailss(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzJobDetailssRequest>,
        ) -> std::result::Result<tonic::Response<super::ListQrtzJobDetailssResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsService/ListQrtzJobDetailss",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsService",
                "ListQrtzJobDetailss",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_qrtz_job_details(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetails>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsService/GetQrtzJobDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsService",
                "GetQrtzJobDetails",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_qrtz_job_details(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetails>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsService/CreateQrtzJobDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsService",
                "CreateQrtzJobDetails",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_qrtz_job_details(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetails>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsService/UpdateQrtzJobDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsService",
                "UpdateQrtzJobDetails",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_qrtz_job_details(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsService/DeleteQrtzJobDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsService",
                "DeleteQrtzJobDetails",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_job_details_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzJobDetailsServiceServer.
    #[async_trait]
    pub trait QrtzJobDetailsService: Send + Sync + 'static {
        async fn list_qrtz_job_detailss(
            &self,
            request: tonic::Request<super::ListQrtzJobDetailssRequest>,
        ) -> std::result::Result<tonic::Response<super::ListQrtzJobDetailssResponse>, tonic::Status>;
        async fn get_qrtz_job_details(
            &self,
            request: tonic::Request<super::GetQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetails>, tonic::Status>;
        async fn create_qrtz_job_details(
            &self,
            request: tonic::Request<super::CreateQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetails>, tonic::Status>;
        async fn update_qrtz_job_details(
            &self,
            request: tonic::Request<super::UpdateQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetails>, tonic::Status>;
        async fn delete_qrtz_job_details(
            &self,
            request: tonic::Request<super::DeleteQrtzJobDetailsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzJobDetailsServiceServer<T: QrtzJobDetailsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzJobDetailsService> QrtzJobDetailsServiceServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(
            mut self,
            limit: usize,
        ) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(
            mut self,
            limit: usize,
        ) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QrtzJobDetailsServiceServer<T>
    where
        T: QrtzJobDetailsService,
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
        fn call(
            &mut self,
            req: http::Request<B>,
        ) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/qrtz_job_details.QrtzJobDetailsService/ListQrtzJobDetailss" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzJobDetailssSvc<T: QrtzJobDetailsService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsService>
                        tonic::server::UnaryService<super::ListQrtzJobDetailssRequest>
                        for ListQrtzJobDetailssSvc<T>
                    {
                        type Response = super::ListQrtzJobDetailssResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQrtzJobDetailssRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_qrtz_job_detailss(request).await };
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
                        let method = ListQrtzJobDetailssSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsService/GetQrtzJobDetails" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzJobDetailsSvc<T: QrtzJobDetailsService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsService>
                        tonic::server::UnaryService<super::GetQrtzJobDetailsRequest>
                        for GetQrtzJobDetailsSvc<T>
                    {
                        type Response = super::QrtzJobDetails;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzJobDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_qrtz_job_details(request).await };
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
                        let method = GetQrtzJobDetailsSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsService/CreateQrtzJobDetails" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzJobDetailsSvc<T: QrtzJobDetailsService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsService>
                        tonic::server::UnaryService<super::CreateQrtzJobDetailsRequest>
                        for CreateQrtzJobDetailsSvc<T>
                    {
                        type Response = super::QrtzJobDetails;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateQrtzJobDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_qrtz_job_details(request).await };
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
                        let method = CreateQrtzJobDetailsSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsService/UpdateQrtzJobDetails" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzJobDetailsSvc<T: QrtzJobDetailsService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsService>
                        tonic::server::UnaryService<super::UpdateQrtzJobDetailsRequest>
                        for UpdateQrtzJobDetailsSvc<T>
                    {
                        type Response = super::QrtzJobDetails;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateQrtzJobDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_qrtz_job_details(request).await };
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
                        let method = UpdateQrtzJobDetailsSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsService/DeleteQrtzJobDetails" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzJobDetailsSvc<T: QrtzJobDetailsService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsService>
                        tonic::server::UnaryService<super::DeleteQrtzJobDetailsRequest>
                        for DeleteQrtzJobDetailsSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteQrtzJobDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_qrtz_job_details(request).await };
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
                        let method = DeleteQrtzJobDetailsSvc(inner);
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
    impl<T: QrtzJobDetailsService> Clone for QrtzJobDetailsServiceServer<T> {
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
    impl<T: QrtzJobDetailsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzJobDetailsService> tonic::server::NamedService for QrtzJobDetailsServiceServer<T> {
        const NAME: &'static str = "qrtz_job_details.QrtzJobDetailsService";
    }
}
