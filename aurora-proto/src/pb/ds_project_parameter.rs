#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectParameter {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub param_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub param_value: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub code: i64,
    #[prost(int64, tag = "5")]
    pub project_code: i64,
    #[prost(int32, optional, tag = "6")]
    pub user_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectParametersRequest {
    /// The maximum number of items to return.
    #[prost(int64, tag = "1")]
    pub page_size: i64,
    #[prost(int64, tag = "2")]
    pub page_num: i64,
    #[prost(string, optional, tag = "3")]
    pub search_val: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "4")]
    pub project_code: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectParametersResponse {
    /// The field name should match the noun "ProjectParameter" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub total_list: ::prost::alloc::vec::Vec<ProjectParameter>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
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
pub struct GetProjectParameterRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectParameterRequest {
    /// The ProjectParameter resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub project_parameter: ::core::option::Option<ProjectParameter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectParameterRequest {
    #[prost(string, tag = "1")]
    pub param_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub param_value: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub code: i64,
    #[prost(int64, tag = "4")]
    pub project_code: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectParameterRequest {
    /// The resource name of the ProjectParameter to be deleted.
    #[prost(int64, tag = "1")]
    pub code: i64,
    #[prost(int64, tag = "2")]
    pub project_code: i64,
}
/// Generated client implementations.
pub mod project_parameter_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct ProjectParameterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectParameterServiceClient<tonic::transport::Channel> {
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
    impl<T> ProjectParameterServiceClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> ProjectParameterServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<<T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody>,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error: Into<StdError> + Send + Sync,
        {
            ProjectParameterServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_project_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectParametersRequest>,
        ) -> std::result::Result<tonic::Response<super::ListProjectParametersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_project_parameter.ProjectParameterService/ListProjectParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_project_parameter.ProjectParameterService",
                "ListProjectParameters",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_project_parameter(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<super::ProjectParameter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_project_parameter.ProjectParameterService/GetProjectParameter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_project_parameter.ProjectParameterService",
                "GetProjectParameter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_project_parameter(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<super::ProjectParameter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_project_parameter.ProjectParameterService/CreateProjectParameter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_project_parameter.ProjectParameterService",
                "CreateProjectParameter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_project_parameter(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<super::ProjectParameter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_project_parameter.ProjectParameterService/UpdateProjectParameter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_project_parameter.ProjectParameterService",
                "UpdateProjectParameter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_project_parameter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_project_parameter.ProjectParameterService/DeleteProjectParameter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_project_parameter.ProjectParameterService",
                "DeleteProjectParameter",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod project_parameter_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProjectParameterServiceServer.
    #[async_trait]
    pub trait ProjectParameterService: Send + Sync + 'static {
        async fn list_project_parameters(
            &self,
            request: tonic::Request<super::ListProjectParametersRequest>,
        ) -> std::result::Result<tonic::Response<super::ListProjectParametersResponse>, tonic::Status>;
        async fn get_project_parameter(
            &self,
            request: tonic::Request<super::GetProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<super::ProjectParameter>, tonic::Status>;
        async fn create_project_parameter(
            &self,
            request: tonic::Request<super::CreateProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<super::ProjectParameter>, tonic::Status>;
        async fn update_project_parameter(
            &self,
            request: tonic::Request<super::UpdateProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<super::ProjectParameter>, tonic::Status>;
        async fn delete_project_parameter(
            &self,
            request: tonic::Request<super::DeleteProjectParameterRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct ProjectParameterServiceServer<T: ProjectParameterService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProjectParameterService> ProjectParameterServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProjectParameterServiceServer<T>
    where
        T: ProjectParameterService,
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
                "/ds_project_parameter.ProjectParameterService/ListProjectParameters" => {
                    #[allow(non_camel_case_types)]
                    struct ListProjectParametersSvc<T: ProjectParameterService>(pub Arc<T>);
                    impl<T: ProjectParameterService> tonic::server::UnaryService<super::ListProjectParametersRequest>
                        for ListProjectParametersSvc<T>
                    {
                        type Response = super::ListProjectParametersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProjectParametersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_project_parameters(request).await };
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
                        let method = ListProjectParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_project_parameter.ProjectParameterService/GetProjectParameter" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectParameterSvc<T: ProjectParameterService>(pub Arc<T>);
                    impl<T: ProjectParameterService> tonic::server::UnaryService<super::GetProjectParameterRequest>
                        for GetProjectParameterSvc<T>
                    {
                        type Response = super::ProjectParameter;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::GetProjectParameterRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_project_parameter(request).await };
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
                        let method = GetProjectParameterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_project_parameter.ProjectParameterService/CreateProjectParameter" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProjectParameterSvc<T: ProjectParameterService>(pub Arc<T>);
                    impl<T: ProjectParameterService> tonic::server::UnaryService<super::CreateProjectParameterRequest>
                        for CreateProjectParameterSvc<T>
                    {
                        type Response = super::ProjectParameter;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProjectParameterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_project_parameter(request).await };
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
                        let method = CreateProjectParameterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_project_parameter.ProjectParameterService/UpdateProjectParameter" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectParameterSvc<T: ProjectParameterService>(pub Arc<T>);
                    impl<T: ProjectParameterService> tonic::server::UnaryService<super::UpdateProjectParameterRequest>
                        for UpdateProjectParameterSvc<T>
                    {
                        type Response = super::ProjectParameter;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProjectParameterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_project_parameter(request).await };
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
                        let method = UpdateProjectParameterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_project_parameter.ProjectParameterService/DeleteProjectParameter" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProjectParameterSvc<T: ProjectParameterService>(pub Arc<T>);
                    impl<T: ProjectParameterService> tonic::server::UnaryService<super::DeleteProjectParameterRequest>
                        for DeleteProjectParameterSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteProjectParameterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_project_parameter(request).await };
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
                        let method = DeleteProjectParameterSvc(inner);
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
    impl<T: ProjectParameterService> Clone for ProjectParameterServiceServer<T> {
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
    impl<T: ProjectParameterService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProjectParameterService> tonic::server::NamedService for ProjectParameterServiceServer<T> {
        const NAME: &'static str = "ds_project_parameter.ProjectParameterService";
    }
}
