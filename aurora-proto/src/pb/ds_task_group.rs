#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsTaskGroup {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, tag = "4")]
    pub group_size: i32,
    #[prost(int64, optional, tag = "5")]
    pub project_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "6")]
    pub use_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub user_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsTaskGroupsRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsTaskGroupsResponse {
    /// The field name should match the noun "DsTaskGroup" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_task_groups: ::prost::alloc::vec::Vec<DsTaskGroup>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsTaskGroupRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsTaskGroupRequest {
    /// The parent resource name where the DsTaskGroup is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsTaskGroup id to use for this DsTaskGroup.
    #[prost(string, tag = "2")]
    pub ds_task_group_id: ::prost::alloc::string::String,
    /// The DsTaskGroup resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_task_group: ::core::option::Option<DsTaskGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsTaskGroupRequest {
    /// The DsTaskGroup resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_task_group: ::core::option::Option<DsTaskGroup>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsTaskGroupRequest {
    /// The resource name of the DsTaskGroup to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_task_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsTaskGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsTaskGroupServiceClient<tonic::transport::Channel> {
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
    impl<T> DsTaskGroupServiceClient<T>
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
        ) -> DsTaskGroupServiceClient<InterceptedService<T, F>>
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
            DsTaskGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_ds_task_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsTaskGroupsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsTaskGroupsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_group.DsTaskGroupService/ListDsTaskGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_group.DsTaskGroupService",
                "ListDsTaskGroups",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ds_task_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_group.DsTaskGroupService/GetDsTaskGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_group.DsTaskGroupService",
                "GetDsTaskGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_ds_task_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_group.DsTaskGroupService/CreateDsTaskGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_group.DsTaskGroupService",
                "CreateDsTaskGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ds_task_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_group.DsTaskGroupService/UpdateDsTaskGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_group.DsTaskGroupService",
                "UpdateDsTaskGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_ds_task_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_task_group.DsTaskGroupService/DeleteDsTaskGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_task_group.DsTaskGroupService",
                "DeleteDsTaskGroup",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_task_group_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsTaskGroupServiceServer.
    #[async_trait]
    pub trait DsTaskGroupService: Send + Sync + 'static {
        async fn list_ds_task_groups(
            &self,
            request: tonic::Request<super::ListDsTaskGroupsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsTaskGroupsResponse>, tonic::Status>;
        async fn get_ds_task_group(
            &self,
            request: tonic::Request<super::GetDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskGroup>, tonic::Status>;
        async fn create_ds_task_group(
            &self,
            request: tonic::Request<super::CreateDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskGroup>, tonic::Status>;
        async fn update_ds_task_group(
            &self,
            request: tonic::Request<super::UpdateDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::DsTaskGroup>, tonic::Status>;
        async fn delete_ds_task_group(
            &self,
            request: tonic::Request<super::DeleteDsTaskGroupRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsTaskGroupServiceServer<T: DsTaskGroupService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsTaskGroupService> DsTaskGroupServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsTaskGroupServiceServer<T>
    where
        T: DsTaskGroupService,
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
                "/ds_task_group.DsTaskGroupService/ListDsTaskGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsTaskGroupsSvc<T: DsTaskGroupService>(pub Arc<T>);
                    impl<T: DsTaskGroupService>
                        tonic::server::UnaryService<super::ListDsTaskGroupsRequest>
                        for ListDsTaskGroupsSvc<T>
                    {
                        type Response = super::ListDsTaskGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsTaskGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_task_groups(request).await };
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
                        let method = ListDsTaskGroupsSvc(inner);
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
                "/ds_task_group.DsTaskGroupService/GetDsTaskGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsTaskGroupSvc<T: DsTaskGroupService>(pub Arc<T>);
                    impl<T: DsTaskGroupService>
                        tonic::server::UnaryService<super::GetDsTaskGroupRequest>
                        for GetDsTaskGroupSvc<T>
                    {
                        type Response = super::DsTaskGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsTaskGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_task_group(request).await };
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
                        let method = GetDsTaskGroupSvc(inner);
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
                "/ds_task_group.DsTaskGroupService/CreateDsTaskGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsTaskGroupSvc<T: DsTaskGroupService>(pub Arc<T>);
                    impl<T: DsTaskGroupService>
                        tonic::server::UnaryService<super::CreateDsTaskGroupRequest>
                        for CreateDsTaskGroupSvc<T>
                    {
                        type Response = super::DsTaskGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsTaskGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_ds_task_group(request).await };
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
                        let method = CreateDsTaskGroupSvc(inner);
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
                "/ds_task_group.DsTaskGroupService/UpdateDsTaskGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsTaskGroupSvc<T: DsTaskGroupService>(pub Arc<T>);
                    impl<T: DsTaskGroupService>
                        tonic::server::UnaryService<super::UpdateDsTaskGroupRequest>
                        for UpdateDsTaskGroupSvc<T>
                    {
                        type Response = super::DsTaskGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsTaskGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ds_task_group(request).await };
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
                        let method = UpdateDsTaskGroupSvc(inner);
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
                "/ds_task_group.DsTaskGroupService/DeleteDsTaskGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsTaskGroupSvc<T: DsTaskGroupService>(pub Arc<T>);
                    impl<T: DsTaskGroupService>
                        tonic::server::UnaryService<super::DeleteDsTaskGroupRequest>
                        for DeleteDsTaskGroupSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsTaskGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_ds_task_group(request).await };
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
                        let method = DeleteDsTaskGroupSvc(inner);
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
    impl<T: DsTaskGroupService> Clone for DsTaskGroupServiceServer<T> {
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
    impl<T: DsTaskGroupService> Clone for _Inner<T> {
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
    impl<T: DsTaskGroupService> tonic::server::NamedService for DsTaskGroupServiceServer<T> {
        const NAME: &'static str = "ds_task_group.DsTaskGroupService";
    }
}
