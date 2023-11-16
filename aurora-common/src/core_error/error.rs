use std::{collections::HashMap, str::FromStr};

use axum::{
    http::Extensions,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use serde_json::json;
use tracing::error;
//dolphinscheduler/dolphinscheduler-api/src/main/java/org/apache/dolphinscheduler/api/enums/Status.java
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    SUCCESS(AuroraData),
    InternalServerErrorArgs(AuroraData),
    RequestParamsNotValidError(AuroraData), //(10001, "request parameter {0} is not valid", "请求参数[{0}]无效"),
    TaskTimeoutParamsError(AuroraData), //(10002, "task timeout parameter is not valid", "任务超时参数无效"),
    UserNameExist(AuroraData),          //(10003, "user name already exists", "用户名已存在"),
    UserNameNull(AuroraData),           //(10004, "user name is null", "用户名不能为空"),
    HdfsOperationError(AuroraData),     //(10006, "hdfs operation error", "hdfs操作错误"),
    TaskInstanceNotFound(AuroraData),   //(10008, "task instance not found", "任务实例不存在"),
    OsTenantCodeExist(AuroraData), //(10009, "os tenant code {0} already exists", "操作系统租户[{0}]已存在"),
    UserNotExist(AuroraData),      //(10010, "user {0} not exists", "用户[{0}]不存在"),
    AlertGroupNotExist(AuroraData), //(10011, "alarm group not found", "告警组不存在"),
    AlertGroupExist(AuroraData),   //(10012, "alarm group already exists", "告警组名称已存在"),
    UserNamePasswdError(AuroraData), //(10013, "user name or password error", "用户名或密码错误"),
    LoginSessionFailed(AuroraData), //(10014, "create session failed!", "创建session失败"),
    DatasourceExist(AuroraData),   //(10015, "data source name already exists", "数据源名称已存在"),
    DatasourceConnectFailed(AuroraData), //(10016, "data source connection failed", "建立数据源连接失败"),
    TenantNotExist(AuroraData),          //(10017, "tenant not exists", "租户不存在"),
    ProjectNotFound(AuroraData),         //(10018, "project {0} not found ", "项目[{0}]不存在"),
    ProjectAlreadyExists(AuroraData), //(10019, "project {0} already exists", "项目名称[{0}]已存在"),
    TaskInstanceNotExists(AuroraData), //(10020, "task instance {0} does not exist", "任务实例[{0}]不存在"),
    TaskInstanceNotSubWorkflowInstance(AuroraData), //(10021, "task instance {0} is not sub process instance", "任务实例[{0}]不是子流程实例"),
    ScheduleCronNotExists(AuroraData), //(10022, "scheduler crontab {0} does not exist", "调度配置定时表达式[{0}]不存在"),
    ScheduleCronOnlineForbidUpdate(AuroraData), //(10023, "online status does not allow update operations", "调度配置上线状态不允许修改"),
    ScheduleCronCheckFailed(AuroraData), //(10024, "scheduler crontab expression validation failure: {0}", "调度配置定时表达式验证失败: {0}"),
    MasterNotExists(AuroraData),         //(10025, "master does not exist", "无可用master节点"),
    ScheduleStatusUnknown(AuroraData),   //(10026, "unknown status: {0}", "未知状态: {0}"),
    CreateAlertGroupError(AuroraData),   //(10027, "create alert group error", "创建告警组错误"),
    QueryAllAlertgroupError(AuroraData), //(10028, "query all alertgroup error", "查询告警组错误"),
    ListPagingAlertGroupError(AuroraData), //(10029, "list paging alert group error", "分页查询告警组错误"),
    UpdateAlertGroupError(AuroraData),     //(10030, "update alert group error", "更新告警组错误"),
    DeleteAlertGroupError(AuroraData),     //(10031, "delete alert group error", "删除告警组错误"),
    AlertGroupGrantUserError(AuroraData), //(10032, "alert group grant user error", "告警组授权用户错误"),
    CreateDatasourceError(AuroraData),    //(10033, "create datasource error", "创建数据源错误"),
    UpdateDatasourceError(AuroraData),    //(10034, "update datasource error", "更新数据源错误"),
    QueryDatasourceError(AuroraData),     //(10035, "query datasource error", "查询数据源错误"),
    ConnectDatasourceFailure(AuroraData), //(10036, "connect datasource failure", "建立数据源连接失败"),
    ConnectionTestFailure(AuroraData), //(10037, "connection test failure", "测试数据源连接失败"),
    DeleteDataSourceFailure(AuroraData), //(10038, "delete data source failure", "删除数据源失败"),
    VerifyDatasourceNameFailure(AuroraData), //(10039, "verify datasource name failure", "验证数据源名称失败"),
    UnauthorizedDatasource(AuroraData), //(10040, "unauthorized datasource", "未经授权的数据源"),
    AuthorizedDataSource(AuroraData),   //(10041, "authorized data source", "授权数据源失败"),
    LoginSuccess(AuroraData),           //(10042, "login success", "登录成功"),
    UserLoginFailure(AuroraData),       //(10043, "user login failure", "用户登录失败"),
    ListWorkersError(AuroraData),       //(10044, "list workers error", "查询worker列表错误"),
    ListMastersError(AuroraData),       //(10045, "list masters error", "查询master列表错误"),
    UpdateProjectError(AuroraData),     //(10046, "update project error", "更新项目信息错误"),
    QueryProjectDetailsByCodeError(AuroraData), //(10047, "query project details by code error", "查询项目详细信息错误"),
    CreateProjectError(AuroraData),             //(10048, "create project error", "创建项目错误"),
    LoginUserQueryProjectListPagingError(AuroraData), //(10049, "login user query project list paging error", "分页查询项目列表错误"),
    DeleteProjectError(AuroraData), //(10050, "delete project error", "删除项目错误"),
    QueryUnauthorizedProjectError(AuroraData), //(10051, "query unauthorized project error", "查询未授权项目错误"),
    QueryAuthorizedProject(AuroraData), //(10052, "query authorized project", "查询授权项目错误"),
    QueryQueueListError(AuroraData),    //(10053, "query queue list error", "查询队列列表错误"),
    CreateResourceError(AuroraData),    //(10054, "create resource error", "创建资源错误"),
    UpdateResourceError(AuroraData),    //(10055, "update resource error", "更新资源错误"),
    QueryResourcesListError(AuroraData), //(10056, "query resources list error", "查询资源列表错误"),
    QueryResourcesListPaging(AuroraData), //(10057, "query resources list paging", "分页查询资源列表错误"),
    DeleteResourceError(AuroraData),      //(10058, "delete resource error", "删除资源错误"),
    VerifyResourceByNameAndTypeError(AuroraData), //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
    ViewResourceFileOnLineError(AuroraData), //(10060, "view resource file online error", "查看资源文件错误"),
    CreateResourceFileOnLineError(AuroraData), //(10061, "create resource file online error", "创建资源文件错误"),
    ResourceFileIsEmpty(AuroraData), //(10062, "resource file is empty", "资源文件内容不能为空"),
    EditResourceFileOnLineError(AuroraData), //(10063, "edit resource file online error", "更新资源文件错误"),
    DownloadResourceFileError(AuroraData), //(10064, "download resource file error", "下载资源文件错误"),
    CreateUdfFunctionError(AuroraData), //(10065, "create udf function error", "创建UDF函数错误"),
    ViewUdfFunctionError(AuroraData),   //(10066, "view udf function error", "查询UDF函数错误"),
    UpdateUdfFunctionError(AuroraData), //(10067, "update udf function error", "更新UDF函数错误"),
    QueryUdfFunctionListPagingError(AuroraData), //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
    QueryDatasourceByTypeError(AuroraData), //(10069, "query datasource by type error", "查询数据源信息错误"),
    VerifyUdfFunctionNameError(AuroraData), //(10070, "verify udf function name error", "UDF函数名称验证错误"),
    DeleteUdfFunctionError(AuroraData), //(10071, "delete udf function error", "删除UDF函数错误"),
    AuthorizedFileResourceError(AuroraData), //(10072, "authorized file resource error", "授权资源文件错误"),
    AuthorizeResourceTree(AuroraData), //(10073, "authorize resource tree display error", "授权资源目录树错误"),
    UnauthorizedUdfFunctionError(AuroraData), //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
    AuthorizedUdfFunctionError(AuroraData), //(10075, "authorized udf function error", "授权UDF函数错误"),
    CreateScheduleError(AuroraData),        //(10076, "create schedule error", "创建调度配置错误"),
    UpdateScheduleError(AuroraData),        //(10077, "update schedule error", "更新调度配置错误"),
    PublishScheduleOnlineError(AuroraData), //(10078, "publish schedule online error", "上线调度配置错误"),
    OfflineScheduleError(AuroraData),       //(10079, "offline schedule error", "下线调度配置错误"),
    QueryScheduleListPagingError(AuroraData), //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
    QueryScheduleListError(AuroraData), //(10081, "query schedule list error", "查询调度配置列表错误"),
    QueryTaskListPagingError(AuroraData), //(10082, "query task list paging error", "分页查询任务列表错误"),
    QueryTaskRecordListPagingError(AuroraData), //(10083, "query task record list paging error", "分页查询任务记录错误"),
    CreateTenantError(AuroraData),              //(10084, "create tenant error", "创建租户错误"),
    QueryTenantListPagingError(AuroraData), //(10085, "query tenant list paging error", "分页查询租户列表错误"),
    QueryTenantListError(AuroraData), //(10086, "query tenant list error", "查询租户列表错误"),
    UpdateTenantError(AuroraData),    //(10087, "update tenant error", "更新租户错误"),
    DeleteTenantByIdError(AuroraData), //(10088, "delete tenant by id error", "删除租户错误"),
    VerifyOsTenantCodeError(AuroraData), //(10089, "verify os tenant code error", "操作系统租户验证错误"),
    CreateUserError(AuroraData),         //(10090, "create user error", "创建用户错误"),
    QueryUserListPagingError(AuroraData), //(10091, "query user list paging error", "分页查询用户列表错误"),
    UpdateUserError(AuroraData),          //(10092, "update user error", "更新用户错误"),
    DeleteUserByIdError(AuroraData),      //(10093, "delete user by id error", "删除用户错误"),
    GrantProjectError(AuroraData),        //(10094, "grant project error", "授权项目错误"),
    GrantResourceError(AuroraData),       //(10095, "grant resource error", "授权资源错误"),
    GrantUdfFunctionError(AuroraData),    //(10096, "grant udf function error", "授权UDF函数错误"),
    GrantDatasourceError(AuroraData),     //(10097, "grant datasource error", "授权数据源错误"),
    GetUserInfoError(AuroraData),         //(10098, "get user info error", "获取用户信息错误"),
    UserListError(AuroraData),            //(10099, "user list error", "查询用户列表错误"),
    VerifyUsernameError(AuroraData),      //(10100, "verify username error", "用户名验证错误"),
    UnauthorizedUserError(AuroraData), //(10101, "unauthorized user error", "查询未授权用户错误"),
    AuthorizedUserError(AuroraData),   //(10102, "authorized user error", "查询授权用户错误"),
    QueryTaskInstanceLogError(AuroraData), //(10103, "view task instance log error", "查询任务实例日志错误"),
    DownloadTaskInstanceLogFileError(AuroraData), //(10104, "download task instance log file error", "下载任务日志文件错误"),
    CreateProcessDefinitionError(AuroraData), //(10105, "create process definition error", "创建工作流错误"),
    VerifyProcessDefinitionNameUniqueError(AuroraData), //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
    UpdateProcessDefinitionError(AuroraData), //(10107, "update process definition error", "更新工作流定义错误"),
    ReleaseProcessDefinitionError(AuroraData), //(10108, "release process definition error", "上线工作流错误"),
    QueryDetailOfProcessDefinitionError(AuroraData), //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
    QueryProcessDefinitionList(AuroraData), //(10110, "query process definition list", "查询工作流列表错误"),
    EncapsulationTreeviewStructureError(AuroraData), //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
    GetTasksListByProcessDefinitionIdError(AuroraData), //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
    QueryProcessInstanceListPagingError(AuroraData), //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
    QueryTaskListByProcessInstanceIdError(AuroraData), //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
    UpdateProcessInstanceError(AuroraData), //(10115, "update process instance error", "更新工作流实例错误"),
    QueryProcessInstanceByIdError(AuroraData), //(10116, "query process instance by id error", "查询工作流实例错误"),
    DeleteProcessInstanceByIdError(AuroraData), //(10117, "delete process instance by id error", "删除工作流实例错误"),
    QuerySubProcessInstanceDetailInfoByTaskIdError(AuroraData), //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
    QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(AuroraData), //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
    QueryProcessInstanceAllVariablesError(AuroraData), //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
    EncapsulationProcessInstanceGanttStructureError(AuroraData), //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
    QueryProcessDefinitionListPagingError(AuroraData), //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
    SignOutError(AuroraData),                          //(10123, "sign out error", "退出错误"),
    OsTenantCodeHasAlreadyExists(AuroraData), //(10124, "os tenant code has already exists", "操作系统租户已存在"),
    IpIsEmpty(AuroraData),                    //(10125, "ip is empty", "IP地址不能为空"),
    ScheduleCronReleaseNeedNotChange(AuroraData), //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
    CreateQueueError(AuroraData),                 //(10127, "create queue error", "创建队列错误"),
    QueueNotExist(AuroraData), //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
    QueueValueExist(AuroraData), //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
    QueueNameExist(AuroraData), //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
    UpdateQueueError(AuroraData), //(10131, "update queue error", "更新队列信息错误"),
    NeedNotUpdateQueue(AuroraData), //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
    VerifyQueueError(AuroraData),   //(10133, "verify queue error", "验证队列信息错误"),
    NameNull(AuroraData),           //(10134, "name must be not null", "名称不能为空"),
    NameExist(AuroraData),          //(10135, "name {0} already exists", "名称[{0}]已存在"),
    SaveError(AuroraData),          //(10136, "save error", "保存错误"),
    DeleteProjectErrorDefinesNotNull(AuroraData), //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
    BatchDeleteProcessInstanceByIdsError(AuroraData), //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
    PreviewScheduleError(AuroraData), //(10139, "preview schedule error", "预览调度配置错误"),
    ParseToCronExpressionError(AuroraData), //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
    ScheduleStartTimeEndTimeSame(AuroraData), //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
    DeleteTenantByIdFail(AuroraData), //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
    DeleteTenantByIdFailDefines(AuroraData), //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
    DeleteTenantByIdFailUsers(AuroraData), //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
    DeleteWorkerGroupByIdFail(AuroraData), //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
    QueryWorkerGroupFail(AuroraData), //(10146, "query worker group fail ", "查询worker分组失败"),
    DeleteWorkerGroupFail(AuroraData), //(10147, "delete worker group fail ", "删除worker分组失败"),
    UserDisabled(AuroraData),         //(10148, "The current user is disabled", "当前用户已停用"),
    CopyProcessDefinitionError(AuroraData), //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
    MoveProcessDefinitionError(AuroraData), //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
    SwitchProcessDefinitionVersionError(AuroraData), //(10151, "Switch process definition version error", "切换工作流版本出错"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionError(AuroraData), //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(AuroraData), //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
    QueryProcessDefinitionVersionsError(AuroraData), //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
    DeleteProcessDefinitionVersionError(AuroraData), //(10156, "delete process definition version error", "删除工作流历史版本出错"),

    QueryUserCreatedProjectError(AuroraData), //(10157, "query user created project error error", "查询用户创建的项目错误"),
    ProcessDefinitionCodesIsEmpty(AuroraData), //(10158, "process definition codes is empty", "工作流CODES不能为空"),
    BatchCopyProcessDefinitionError(AuroraData), //(10159, "batch copy process definition error", "复制工作流错误"),
    BatchMoveProcessDefinitionError(AuroraData), //(10160, "batch move process definition error", "移动工作流错误"),
    QueryWorkflowLineageError(AuroraData), //(10161, "query workflow lineage error", "查询血缘失败"),
    QueryAuthorizedAndUserCreatedProjectError(AuroraData), //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
    DeleteProcessDefinitionByCodeFail(AuroraData), //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
    CheckOsTenantCodeError(AuroraData), //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
    ForceTaskSuccessError(AuroraData), //(10165, "force task success error", "强制成功任务实例错误"),
    TaskInstanceStateOperationError(AuroraData), //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
    DatasourceTypeNotExist(AuroraData), //(10167, "data source type not exist", "数据源类型不存在"),
    ProcessDefinitionNameExist(AuroraData), //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
    DatasourceDbTypeIllegal(AuroraData), //(10169, "datasource type illegal", "数据源类型参数不合法"),
    DatasourcePortIllegal(AuroraData), //(10170, "datasource port illegal", "数据源端口参数不合法"),
    DatasourceOtherParamsIllegal(AuroraData), //(10171, "datasource other params illegal", "数据源其他参数不合法"),
    DatasourceNameIllegal(AuroraData), //(10172, "datasource name illegal", "数据源名称不合法"),
    DatasourceHostIllegal(AuroraData), //(10173, "datasource host illegal", "数据源HOST不合法"),
    DeleteWorkerGroupNotExist(AuroraData), //(10174, "delete worker group not exist ", "删除worker分组不存在"),
    CreateWorkerGroupForbiddenInDocker(AuroraData), //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
    DeleteWorkerGroupForbiddenInDocker(AuroraData), //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
    WorkerAddressInvalid(AuroraData), //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
    QueryWorkerAddressListFail(AuroraData), //(10178, "query worker address list fail ", "查询worker地址列表失败"),
    TransformProjectOwnership(AuroraData), //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
    QueryAlertGroupError(AuroraData),      //(10180, "query alert group error", "查询告警组错误"),
    CurrentLoginUserTenantNotExist(AuroraData), //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
    RevokeProjectError(AuroraData), //(10182, "revoke project error", "撤销项目授权错误"),
    QueryAuthorizedUser(AuroraData), //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
    ProjectNotExist(AuroraData), //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
    TaskInstanceHostIsNull(AuroraData), //(10191, "task instance host is null", "任务实例host为空"),
    QueryExecutingWorkflowError(AuroraData), //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

    UdfFunctionNotExist(AuroraData), //(20001, "UDF function not found", "UDF函数不存在"),
    UdfFunctionExists(AuroraData),   //(20002, "UDF function already exists", "UDF函数已存在"),
    ResourceNotExist(AuroraData),    //(20004, "resource not exist", "资源不存在"),
    ResourceExist(AuroraData),       //(20005, "resource already exists", "资源已存在"),
    ResourceSuffixNotSupportView(AuroraData), //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
    ResourceSizeExceedLimit(AuroraData), //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
    ResourceSuffixForbidChange(AuroraData), //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
    UdfResourceSuffixNotJar(AuroraData), //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
    HdfsCopyFail(AuroraData), //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
    ResourceFileExist(AuroraData), //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
    ResourceFileNotExist(AuroraData), //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
    UdfResourceIsBound(AuroraData), //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
    ResourceIsUsed(AuroraData), //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
    ParentResourceNotExist(AuroraData), //(20015, "parent resource not exist", "父资源文件不存在"),
    ResourceNotExistOrNoPermission(AuroraData), //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
    ResourceIsAuthorized(AuroraData), //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

    UserNoOperationPerm(AuroraData), //(30001, "user has no operation privilege", "当前用户没有操作权限"),
    UserNoOperationProjectPerm(AuroraData), //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

    ProcessInstanceNotExist(AuroraData), //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
    ProcessInstanceExist(AuroraData), //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
    ProcessDefineNotExist(AuroraData), //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
    ProcessDefineNotRelease(AuroraData), //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
    SubProcessDefineNotRelease(AuroraData), //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
    ProcessInstanceAlreadyChanged(AuroraData), //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
    ProcessInstanceStateOperationError(AuroraData), //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
    SubProcessInstanceNotExist(AuroraData), //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
    ProcessDefineNotAllowedEdit(AuroraData), //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
    ProcessInstanceExecutingCommand(AuroraData), //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
    ProcessInstanceNotSubProcessInstance(AuroraData), //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
    TaskInstanceStateCountError(AuroraData), //(50011, "task instance state count error", "查询各状态任务实例数错误"),
    CountProcessInstanceStateError(AuroraData), //(50012, "count process instance state error", "查询各状态流程实例数错误"),
    CountProcessDefinitionUserError(AuroraData), //(50013, "count process definition user error", "查询各用户流程定义数错误"),
    StartProcessInstanceError(AuroraData), //(50014, "start process instance error", "运行工作流实例错误"),
    BatchStartProcessInstanceError(AuroraData), //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
    ProcessInstanceError(AuroraData), //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
    ExecuteProcessInstanceError(AuroraData), //(50015, "execute process instance error", "操作工作流实例错误"),
    CheckProcessDefinitionError(AuroraData), //(50016, "check process definition error", "工作流定义错误"),
    QueryRecipientsAndCopyersByProcessDefinitionError(AuroraData), //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误"),
    DataIsNotValid(AuroraData), //(50017, "data {0} not valid", "数据[{0}]无效"),
    DataIsNull(AuroraData),     //(50018, "data {0} is null", "数据[{0}]不能为空"),
    ProcessNodeHasCycle(AuroraData), //(50019, "process node has cycle", "流程节点间存在循环依赖"),
    ProcessNodeSParameterInvalid(AuroraData), //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效"),
    ProcessDefineStateOnline(AuroraData), //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线"),
    DeleteProcessDefineByCodeError(AuroraData), //(50022, "delete process definition by code error", "删除工作流定义错误"),
    ScheduleCronStateOnline(AuroraData), //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线"),
    DeleteScheduleCronByIdError(AuroraData), //(50024, "delete schedule by id error", "删除调度配置错误"),
    BatchDeleteProcessDefineError(AuroraData), //(50025, "batch delete process definition error", "批量删除工作流定义错误"),
    BatchDeleteProcessDefineByCodesError(AuroraData), //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误"),
    DeleteProcessDefineByCodesError(AuroraData), //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误"),
    TenantNotSuitable(AuroraData), //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户"),
    ExportProcessDefineByIdError(AuroraData), //(50028, "export process definition by id error", "导出工作流定义错误"),
    BatchExportProcessDefineByIdsError(AuroraData), //(50028, "batch export process definition by ids error", "批量导出工作流定义错误"),
    ImportProcessDefineError(AuroraData), //(50029, "import process definition error", "导入工作流定义错误"),
    TaskDefineNotExist(AuroraData), //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在"),
    CreateProcessTaskRelationError(AuroraData), //(50032, "create process task relation error", "创建工作流任务关系错误"),
    ProcessTaskRelationNotExist(AuroraData), //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在"),
    ProcessTaskRelationExist(AuroraData), //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]"),
    ProcessDagIsEmpty(AuroraData),        //(50035, "process dag is empty", "工作流dag是空"),
    CheckProcessTaskRelationError(AuroraData), //(50036, "check process task relation error", "工作流任务关系参数错误"),
    CreateTaskDefinitionError(AuroraData), //(50037, "create task definition error", "创建任务错误"),
    UpdateTaskDefinitionError(AuroraData), //(50038, "update task definition error", "更新任务定义错误"),
    QueryTaskDefinitionVersionsError(AuroraData), //(50039, "query task definition versions error", "查询任务历史版本信息出错"),
    SwitchTaskDefinitionVersionError(AuroraData), //(50040, "Switch task definition version error", "切换任务版本出错"),
    DeleteTaskDefinitionVersionError(AuroraData), //(50041, "delete task definition version error", "删除任务历史版本出错"),
    DeleteTaskDefineByCodeError(AuroraData), //(50042, "delete task definition by code error", "删除任务定义错误"),
    QueryDetailOfTaskDefinitionError(AuroraData), //(50043, "query detail of task definition error", "查询任务详细信息错误"),
    QueryTaskDefinitionListPagingError(AuroraData), //(50044, "query task definition list paging error", "分页查询任务定义列表错误"),
    TaskDefinitionNameExisted(AuroraData), //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在"),
    ReleaseTaskDefinitionError(AuroraData), //(50046, "release task definition error", "上线任务错误"),
    MoveProcessTaskRelationError(AuroraData), //(50047, "move process task relation error", "移动任务到其他工作流错误"),
    DeleteTaskProcessRelationError(AuroraData), //(50048, "delete process task relation error", "删除工作流任务关系错误"),
    QueryTaskProcessRelationError(AuroraData), //(50049, "query process task relation error", "查询工作流任务关系错误"),
    TaskDefineStateOnline(AuroraData), //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线"),
    TaskHasDownstream(AuroraData), //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖"),
    TaskHasUpstream(AuroraData), //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖"),
    MainTableUsingVersion(AuroraData), //(50053, "the version that the master table is using", "主表正在使用该版本"),
    ProjectProcessNotMatch(AuroraData), //(50054, "the project and the process is not match", "项目和工作流不匹配"),
    DeleteEdgeError(AuroraData),        //(50055, "delete edge error", "删除工作流任务连接线错误"),
    NotSupportUpdateTaskDefinition(AuroraData), //(50056, "task state does not support modification", "当前任务不支持修改"),
    NotSupportCopyTaskType(AuroraData), //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]"),
    HdfsNotStartup(AuroraData),         //(60001, "hdfs not startup", "hdfs未启用"),
    StorageNotStartup(AuroraData),      //(60002, "storage not startup", "存储未启用"),
    S3CannotRename(AuroraData), //(60003, "directory cannot be renamed", "S3无法重命名文件夹"),
    /**
     * for monitor
     */
    QueryDatabaseStateError(AuroraData), //(70001, "query database state error", "查询数据库状态错误"),

    CreateAccessTokenError(AuroraData), //(70010, "create access token error", "创建访问token错误"),
    GenerateTokenError(AuroraData),     //(70011, "generate token error", "生成token错误"),
    QueryAccesstokenListPagingError(AuroraData), //(70012, "query access token list paging error", "分页查询访问token列表错误"),
    UpdateAccessTokenError(AuroraData), //(70013, "update access token error", "更新访问token错误"),
    DeleteAccessTokenError(AuroraData), //(70014, "delete access token error", "删除访问token错误"),
    AccessTokenNotExist(AuroraData),    //(70015, "access token not exist", "访问token不存在"),
    QueryAccesstokenByUserError(AuroraData), //(70016, "query access token by user error", "查询访问指定用户的token错误"),

    CommandStateCountError(AuroraData), //(80001, "task instance state count error", "查询各状态任务实例数错误"),
    NegativeSizeNumberError(AuroraData), //(80002, "query size number error", "查询size错误"),
    StartTimeBiggerThanEndTimeError(AuroraData), //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误"),
    QueueCountError(AuroraData),                 //(90001, "queue count error", "查询队列数据错误"),

    KerberosStartupState(AuroraData), //(100001, "get kerberos startup state error", "获取kerberos启动状态错误"),

    // audit log
    QueryAuditLogListPaging(AuroraData), //(10057, "query resources list paging", "分页查询资源列表错误"),

    //plugin
    PluginNotAUiComponent(AuroraData), //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件"),
    QueryPluginsResultIsNull(AuroraData), //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功"),
    QueryPluginsError(AuroraData),        //(110003, "query plugins error", "查询插件错误"),
    QueryPluginDetailResultIsNull(AuroraData), //(110004, "query plugin detail result is null", "查询插件详情结果为空"),

    UpdateAlertPluginInstanceError(AuroraData), //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误"),
    DeleteAlertPluginInstanceError(AuroraData), //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误"),
    GetAlertPluginInstanceError(AuroraData), //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误"),
    CreateAlertPluginInstanceError(AuroraData), //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误"),
    QueryAllAlertPluginInstanceError(AuroraData), //(110009, "query all alert plugin instance error", "查询所有告警实例失败"),
    PluginInstanceAlreadyExit(AuroraData), //(110010, "plugin instance already exit", "该告警插件实例已存在"),
    ListPagingAlertPluginInstanceError(AuroraData), //(110011, "query plugin instance page error", "分页查询告警实例失败"),
    DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(AuroraData), //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组"),
    ProcessDefinitionVersionIsUsed(AuroraData), //(110013, "this process definition version is used", "此工作流定义版本被使用"),

    CreateEnvironmentError(AuroraData), //(120001, "create environment error", "创建环境失败"),
    EnvironmentNameExists(AuroraData), //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在"),
    EnvironmentNameIsNull(AuroraData), //(120003, "this environment name shouldn't be empty.", "环境名称不能为空"),
    EnvironmentConfigIsNull(AuroraData), //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空"),
    UpdateEnvironmentError(AuroraData), //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败"),
    DeleteEnvironmentError(AuroraData), //(120006, "delete environment error", "删除环境信息失败"),
    DeleteEnvironmentRelatedTaskExists(AuroraData), //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息"),
    QueryEnvironmentByNameError(AuroraData), //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在"),
    QueryEnvironmentByCodeError(AuroraData), //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在"),
    QueryEnvironmentError(AuroraData), //(1200010, "login user query environment error", "分页查询环境列表错误"),
    VerifyEnvironmentError(AuroraData), //(1200011, "verify environment error", "验证环境信息错误"),
    GetRuleFormCreateJsonError(AuroraData), //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误"),
    QueryRuleListPagingError(AuroraData), //(1200013, "query rule list paging error", "获取规则分页列表错误"),
    QueryRuleListError(AuroraData),       //(1200014, "query rule list error", "获取规则列表错误"),
    QueryRuleInputEntryListError(AuroraData), //(1200015, "query rule list error", "获取规则列表错误"),
    QueryExecuteResultListPagingError(AuroraData), //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误"),
    GetDatasourceOptionsError(AuroraData), //(1200017, "get datasource options error", "获取数据源Options错误"),
    GetDatasourceTablesError(AuroraData), //(1200018, "get datasource tables error", "获取数据源表列表错误"),
    GetDatasourceTableColumnsError(AuroraData), //(1200019, "get datasource table columns error", "获取数据源表列名错误"),
    TaskGroupNameExist(AuroraData), //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用"),
    TaskGroupSizeError(AuroraData), //(130002, "task group size error", "任务组大小应该为大于1的整数"),
    TaskGroupStatusError(AuroraData), //(130003, "task group status error", "任务组已经被关闭"),
    TaskGroupFull(AuroraData),      //(130004, "task group is full", "任务组已经满了"),
    TaskGroupUsedSizeError(AuroraData), //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化"),
    TaskGroupQueueReleaseError(AuroraData), //(130006, "failed to release task group queue", "任务组资源释放时出现了错误"),
    TaskGroupQueueAwakeError(AuroraData), //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误"),
    CreateTaskGroupError(AuroraData),     //(130008, "create task group error", "创建任务组错误"),
    UpdateTaskGroupError(AuroraData), //(130009, "update task group list error", "更新任务组错误"),
    QueryTaskGroupListError(AuroraData), //(130010, "query task group list error", "查询任务组列表错误"),
    CloseTaskGroupError(AuroraData),     //(130011, "close task group error", "关闭任务组错误"),
    StartTaskGroupError(AuroraData),     //(130012, "start task group error", "启动任务组错误"),
    QueryTaskGroupQueueListError(AuroraData), //(130013, "query task group queue list error", "查询任务组队列列表错误"),
    TaskGroupCacheStartFailed(AuroraData), //(130014, "cache start failed", "任务组相关的缓存启动失败"),
    EnvironmentWorkerGroupsIsInvalid(AuroraData), //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误"),
    UpdateEnvironmentWorkerGroupRelationError(AuroraData), //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中"),
    TaskGroupQueueAlreadyStart(AuroraData), //(130017, "task group queue already start", "节点已经获取任务组资源"),
    TaskGroupStatusClosed(AuroraData), //(130018, "The task group has been closed.", "任务组已经被关闭"),
    TaskGroupStatusOpened(AuroraData), //(130019, "The task group has been opened.", "任务组已经被开启"),
    NotAllowToDisableOwnAccount(AuroraData), //(130020, "Not allow to disable your own account", "不能停用自己的账号"),
    NotAllowToDeleteDefaultAlarmGroup(AuroraData), //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组"),
    TimeZoneIllegal(AuroraData), //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法"),

    QueryK8sNamespaceListPagingError(AuroraData), //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误"),
    K8sNamespaceExist(AuroraData), //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在"),
    CreateK8sNamespaceError(AuroraData), //(1300003, "create k8s namespace error", "创建k8s命名空间错误"),
    UpdateK8sNamespaceError(AuroraData), //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误"),
    K8sNamespaceNotExist(AuroraData), //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在"),
    K8sClientOpsError(AuroraData), //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]"),
    VerifyK8sNamespaceError(AuroraData), //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误"),
    DeleteK8sNamespaceByIdError(AuroraData), //(1300008, "delete k8s namespace by id error", "删除命名空间错误"),
    VerifyParameterNameFailed(AuroraData), //(1300009, "The file name verify failed", "文件命名校验失败"),
    StoreOperateCreateError(AuroraData), //(1300010, "create the resource failed", "存储操作失败"),
    GrantK8sNamespaceError(AuroraData),  //(1300011, "grant namespace error", "授权资源错误"),
    QueryUnauthorizedNamespaceError(AuroraData), //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误"),
    QueryAuthorizedNamespaceError(AuroraData), //(1300013, "query authorized namespace error", "查询授权命名空间错误"),
    QueryCanUseK8sClusterError(AuroraData), //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误"),
    ResourceFullNameTooLongError(AuroraData), //(1300015, "resource's fullname is too long error", "资源文件名过长"),
    TenantFullNameTooLongError(AuroraData), //(1300016, "tenant's fullname is too long error", "租户名过长");
}
impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        error!("{:<12} - model::Error {val:?}", "FROM_JSON");
        Self::InternalServerErrorArgs(AuroraData::String(val.to_string()))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - model::Error {self:?}", "INTO_RES");
        let error: AuroraErrorInfo = self.into();
        let mut ext = Extensions::new();
        ext.insert(error);
        ext.into_response()
    }
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::SUCCESS(_) => tonic::Status::new(tonic::Code::Ok, "success"),
            _ => {
                let code = tonic::Code::Unknown;

                let info: AuroraErrorInfo = value.into();
                let mut metadata = tonic::metadata::MetadataMap::new();
                metadata.insert("error_code", format!("{}", info.code).parse().unwrap());
                metadata.insert("cn_msg", info.cn_msg.parse().unwrap());
                metadata.insert("en_msg", info.en_msg.parse().unwrap());
                let message: String = info.into();
                tonic::Status::with_metadata(code, message, metadata)
            }
        }
    }
}

impl From<AuroraErrorInfo> for String {
    fn from(value: AuroraErrorInfo) -> Self {
        serde_json::to_string(&value).unwrap()
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::SUCCESS(AuroraData::Null)
    }
}

impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        if value.code() == tonic::Code::Ok {
            return Error::SUCCESS(AuroraData::Null);
        }
        if value.code() == tonic::Code::Internal {
            return Error::InternalServerErrorArgs(AuroraData::Null);
        }
        let code = value.code();
        if code == tonic::Code::Unknown {
            let message = value.message().split('~');
            let mut map = HashMap::new();
            for m in message {
                let kv = m.split('|').collect::<Vec<&str>>();
                if kv.len() == 2 {
                    map.insert(kv[0].to_string(), kv[1].to_string());
                }
            }

            let error_code = String::from("50000");
            let cn_msg = String::from("未知错误");
            let en_msg = String::from("unknown error");
            let error_data = String::from("");
            let error_code = map.get("code").unwrap_or(&error_code);
            let cn_msg = map.get("cn_msg").unwrap_or(&cn_msg);
            let en_msg = map.get("en_msg").unwrap_or(&en_msg);
            let error_data = map.get("error_data").unwrap_or(&error_data);
            error!(
                "error_code:{},cn_msg:{},en_msg:{},error_data:{}",
                error_code, cn_msg, en_msg, error_data
            );
            let error_code: i32 = error_code.parse().unwrap();
            let error = AuroraErrorInfo {
                code: error_code,
                cn_msg: cn_msg.to_string(),
                en_msg: en_msg.to_string(),
                error_data: serde_json::from_str(error_data).unwrap(),
            };
            let error: Error = error.into();
            error
        } else {
            Error::InternalServerErrorArgs(AuroraData::Null)
        }
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::SUCCESS(data) => {
                let ss: AuroraErrorInfo = Error::SUCCESS(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::InternalServerErrorArgs(data) => {
                let ss: AuroraErrorInfo = Error::InternalServerErrorArgs(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::RequestParamsNotValidError(data) => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError(data.clone()).into();
                write!(f, "{}", ss)
            }

            Error::TaskTimeoutParamsError(data) => {
                let ss: AuroraErrorInfo = Error::TaskTimeoutParamsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNameExist(data) => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNameNull(data) => {
                let ss: AuroraErrorInfo = Error::UserNameNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::HdfsOperationError(data) => {
                let ss: AuroraErrorInfo = Error::HdfsOperationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotFound(data) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotFound(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::OsTenantCodeExist(data) => {
                let ss: AuroraErrorInfo = Error::OsTenantCodeExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNotExist(data) => {
                let ss: AuroraErrorInfo = Error::UserNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupNotExist(data) => {
                let ss: AuroraErrorInfo = Error::AlertGroupNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupExist(data) => {
                let ss: AuroraErrorInfo = Error::AlertGroupExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNamePasswdError(data) => {
                let ss: AuroraErrorInfo = Error::UserNamePasswdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::LoginSessionFailed(data) => {
                let ss: AuroraErrorInfo = Error::LoginSessionFailed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceExist(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceConnectFailed(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceConnectFailed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TenantNotExist(data) => {
                let ss: AuroraErrorInfo = Error::TenantNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectNotFound(data) => {
                let ss: AuroraErrorInfo = Error::ProjectNotFound(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectAlreadyExists(data) => {
                let ss: AuroraErrorInfo = Error::ProjectAlreadyExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotExists(data) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotSubWorkflowInstance(data) => {
                let ss: AuroraErrorInfo =
                    Error::TaskInstanceNotSubWorkflowInstance(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronNotExists(data) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronNotExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronOnlineForbidUpdate(data) => {
                let ss: AuroraErrorInfo =
                    Error::ScheduleCronOnlineForbidUpdate(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronCheckFailed(data) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronCheckFailed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MasterNotExists(data) => {
                let ss: AuroraErrorInfo = Error::MasterNotExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleStatusUnknown(data) => {
                let ss: AuroraErrorInfo = Error::ScheduleStatusUnknown(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateAlertGroupError(data) => {
                let ss: AuroraErrorInfo = Error::CreateAlertGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAllAlertgroupError(data) => {
                let ss: AuroraErrorInfo = Error::QueryAllAlertgroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListPagingAlertGroupError(data) => {
                let ss: AuroraErrorInfo = Error::ListPagingAlertGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateAlertGroupError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateAlertGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertGroupError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteAlertGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupGrantUserError(data) => {
                let ss: AuroraErrorInfo = Error::AlertGroupGrantUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateDatasourceError(data) => {
                let ss: AuroraErrorInfo = Error::CreateDatasourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateDatasourceError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateDatasourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDatasourceError(data) => {
                let ss: AuroraErrorInfo = Error::QueryDatasourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ConnectDatasourceFailure(data) => {
                let ss: AuroraErrorInfo = Error::ConnectDatasourceFailure(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ConnectionTestFailure(data) => {
                let ss: AuroraErrorInfo = Error::ConnectionTestFailure(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteDataSourceFailure(data) => {
                let ss: AuroraErrorInfo = Error::DeleteDataSourceFailure(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyDatasourceNameFailure(data) => {
                let ss: AuroraErrorInfo = Error::VerifyDatasourceNameFailure(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedDatasource(data) => {
                let ss: AuroraErrorInfo = Error::UnauthorizedDatasource(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedDataSource(data) => {
                let ss: AuroraErrorInfo = Error::AuthorizedDataSource(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::LoginSuccess(data) => {
                let ss: AuroraErrorInfo = Error::LoginSuccess(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserLoginFailure(data) => {
                let ss: AuroraErrorInfo = Error::UserLoginFailure(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListWorkersError(data) => {
                let ss: AuroraErrorInfo = Error::ListWorkersError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListMastersError(data) => {
                let ss: AuroraErrorInfo = Error::ListMastersError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateProjectError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProjectDetailsByCodeError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProjectDetailsByCodeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateProjectError(data) => {
                let ss: AuroraErrorInfo = Error::CreateProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::LoginUserQueryProjectListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::LoginUserQueryProjectListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProjectError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUnauthorizedProjectError(data) => {
                let ss: AuroraErrorInfo = Error::QueryUnauthorizedProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedProject(data) => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedProject(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryQueueListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryQueueListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateResourceError(data) => {
                let ss: AuroraErrorInfo = Error::CreateResourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateResourceError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateResourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryResourcesListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryResourcesListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryResourcesListPaging(data) => {
                let ss: AuroraErrorInfo = Error::QueryResourcesListPaging(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteResourceError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteResourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyResourceByNameAndTypeError(data) => {
                let ss: AuroraErrorInfo =
                    Error::VerifyResourceByNameAndTypeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ViewResourceFileOnLineError(data) => {
                let ss: AuroraErrorInfo = Error::ViewResourceFileOnLineError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateResourceFileOnLineError(data) => {
                let ss: AuroraErrorInfo = Error::CreateResourceFileOnLineError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileIsEmpty(data) => {
                let ss: AuroraErrorInfo = Error::ResourceFileIsEmpty(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EditResourceFileOnLineError(data) => {
                let ss: AuroraErrorInfo = Error::EditResourceFileOnLineError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DownloadResourceFileError(data) => {
                let ss: AuroraErrorInfo = Error::DownloadResourceFileError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::CreateUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ViewUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::ViewUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUdfFunctionListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryUdfFunctionListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDatasourceByTypeError(data) => {
                let ss: AuroraErrorInfo = Error::QueryDatasourceByTypeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyUdfFunctionNameError(data) => {
                let ss: AuroraErrorInfo = Error::VerifyUdfFunctionNameError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedFileResourceError(data) => {
                let ss: AuroraErrorInfo = Error::AuthorizedFileResourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizeResourceTree(data) => {
                let ss: AuroraErrorInfo = Error::AuthorizeResourceTree(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::UnauthorizedUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::AuthorizedUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateScheduleError(data) => {
                let ss: AuroraErrorInfo = Error::CreateScheduleError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateScheduleError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateScheduleError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PublishScheduleOnlineError(data) => {
                let ss: AuroraErrorInfo = Error::PublishScheduleOnlineError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::OfflineScheduleError(data) => {
                let ss: AuroraErrorInfo = Error::OfflineScheduleError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryScheduleListPagingError(data) => {
                let ss: AuroraErrorInfo = Error::QueryScheduleListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryScheduleListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryScheduleListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskListPagingError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTaskListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskRecordListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryTaskRecordListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateTenantError(data) => {
                let ss: AuroraErrorInfo = Error::CreateTenantError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTenantListPagingError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTenantListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTenantListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTenantListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateTenantError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateTenantError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyOsTenantCodeError(data) => {
                let ss: AuroraErrorInfo = Error::VerifyOsTenantCodeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateUserError(data) => {
                let ss: AuroraErrorInfo = Error::CreateUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUserListPagingError(data) => {
                let ss: AuroraErrorInfo = Error::QueryUserListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateUserError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteUserByIdError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteUserByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantProjectError(data) => {
                let ss: AuroraErrorInfo = Error::GrantProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantResourceError(data) => {
                let ss: AuroraErrorInfo = Error::GrantResourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantUdfFunctionError(data) => {
                let ss: AuroraErrorInfo = Error::GrantUdfFunctionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantDatasourceError(data) => {
                let ss: AuroraErrorInfo = Error::GrantDatasourceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetUserInfoError(data) => {
                let ss: AuroraErrorInfo = Error::GetUserInfoError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserListError(data) => {
                let ss: AuroraErrorInfo = Error::UserListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyUsernameError(data) => {
                let ss: AuroraErrorInfo = Error::VerifyUsernameError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedUserError(data) => {
                let ss: AuroraErrorInfo = Error::UnauthorizedUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedUserError(data) => {
                let ss: AuroraErrorInfo = Error::AuthorizedUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskInstanceLogError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTaskInstanceLogError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DownloadTaskInstanceLogFileError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DownloadTaskInstanceLogFileError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::CreateProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyProcessDefinitionNameUniqueError(data) => {
                let ss: AuroraErrorInfo =
                    Error::VerifyProcessDefinitionNameUniqueError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ReleaseProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::ReleaseProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDetailOfProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryDetailOfProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionList(data) => {
                let ss: AuroraErrorInfo = Error::QueryProcessDefinitionList(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EncapsulationTreeviewStructureError(data) => {
                let ss: AuroraErrorInfo =
                    Error::EncapsulationTreeviewStructureError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetTasksListByProcessDefinitionIdError(data) => {
                let ss: AuroraErrorInfo =
                    Error::GetTasksListByProcessDefinitionIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessInstanceListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskListByProcessInstanceIdError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryTaskListByProcessInstanceIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateProcessInstanceError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateProcessInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceByIdError(data) => {
                let ss: AuroraErrorInfo = Error::QueryProcessInstanceByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessInstanceByIdError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProcessInstanceByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QuerySubProcessInstanceDetailInfoByTaskIdError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QuerySubProcessInstanceDetailInfoByTaskIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(
                        data.clone(),
                    )
                    .into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceAllVariablesError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessInstanceAllVariablesError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EncapsulationProcessInstanceGanttStructureError(data) => {
                let ss: AuroraErrorInfo =
                    Error::EncapsulationProcessInstanceGanttStructureError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessDefinitionListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SignOutError(data) => {
                let ss: AuroraErrorInfo = Error::SignOutError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::OsTenantCodeHasAlreadyExists(data) => {
                let ss: AuroraErrorInfo = Error::OsTenantCodeHasAlreadyExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::IpIsEmpty(data) => {
                let ss: AuroraErrorInfo = Error::IpIsEmpty(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronReleaseNeedNotChange(data) => {
                let ss: AuroraErrorInfo =
                    Error::ScheduleCronReleaseNeedNotChange(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateQueueError(data) => {
                let ss: AuroraErrorInfo = Error::CreateQueueError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueNotExist(data) => {
                let ss: AuroraErrorInfo = Error::QueueNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueValueExist(data) => {
                let ss: AuroraErrorInfo = Error::QueueValueExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueNameExist(data) => {
                let ss: AuroraErrorInfo = Error::QueueNameExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateQueueError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateQueueError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NeedNotUpdateQueue(data) => {
                let ss: AuroraErrorInfo = Error::NeedNotUpdateQueue(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyQueueError(data) => {
                let ss: AuroraErrorInfo = Error::VerifyQueueError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NameNull(data) => {
                let ss: AuroraErrorInfo = Error::NameNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NameExist(data) => {
                let ss: AuroraErrorInfo = Error::NameExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SaveError(data) => {
                let ss: AuroraErrorInfo = Error::SaveError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProjectErrorDefinesNotNull(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProjectErrorDefinesNotNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessInstanceByIdsError(data) => {
                let ss: AuroraErrorInfo =
                    Error::BatchDeleteProcessInstanceByIdsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PreviewScheduleError(data) => {
                let ss: AuroraErrorInfo = Error::PreviewScheduleError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ParseToCronExpressionError(data) => {
                let ss: AuroraErrorInfo = Error::ParseToCronExpressionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleStartTimeEndTimeSame(data) => {
                let ss: AuroraErrorInfo = Error::ScheduleStartTimeEndTimeSame(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFail(data) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFailDefines(data) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFailDefines(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFailUsers(data) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFailUsers(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupByIdFail(data) => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupByIdFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkerGroupFail(data) => {
                let ss: AuroraErrorInfo = Error::QueryWorkerGroupFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupFail(data) => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserDisabled(data) => {
                let ss: AuroraErrorInfo = Error::UserDisabled(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CopyProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::CopyProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MoveProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::MoveProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(
                        data.clone(),
                    )
                    .into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(
                        data.clone(),
                    )
                    .into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionVersionsError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessDefinitionVersionsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefinitionVersionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProcessDefinitionVersionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUserCreatedProjectError(data) => {
                let ss: AuroraErrorInfo = Error::QueryUserCreatedProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionCodesIsEmpty(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionCodesIsEmpty(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchCopyProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::BatchCopyProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchMoveProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::BatchMoveProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkflowLineageError(data) => {
                let ss: AuroraErrorInfo = Error::QueryWorkflowLineageError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedAndUserCreatedProjectError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryAuthorizedAndUserCreatedProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefinitionByCodeFail(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProcessDefinitionByCodeFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CheckOsTenantCodeError(data) => {
                let ss: AuroraErrorInfo = Error::CheckOsTenantCodeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ForceTaskSuccessError(data) => {
                let ss: AuroraErrorInfo = Error::ForceTaskSuccessError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceStateOperationError(data) => {
                let ss: AuroraErrorInfo =
                    Error::TaskInstanceStateOperationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceTypeNotExist(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceTypeNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionNameExist(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionNameExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceDbTypeIllegal(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceDbTypeIllegal(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourcePortIllegal(data) => {
                let ss: AuroraErrorInfo = Error::DatasourcePortIllegal(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceOtherParamsIllegal(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceOtherParamsIllegal(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceNameIllegal(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceNameIllegal(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceHostIllegal(data) => {
                let ss: AuroraErrorInfo = Error::DatasourceHostIllegal(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupNotExist(data) => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateWorkerGroupForbiddenInDocker(data) => {
                let ss: AuroraErrorInfo =
                    Error::CreateWorkerGroupForbiddenInDocker(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupForbiddenInDocker(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteWorkerGroupForbiddenInDocker(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::WorkerAddressInvalid(data) => {
                let ss: AuroraErrorInfo = Error::WorkerAddressInvalid(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkerAddressListFail(data) => {
                let ss: AuroraErrorInfo = Error::QueryWorkerAddressListFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TransformProjectOwnership(data) => {
                let ss: AuroraErrorInfo = Error::TransformProjectOwnership(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAlertGroupError(data) => {
                let ss: AuroraErrorInfo = Error::QueryAlertGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CurrentLoginUserTenantNotExist(data) => {
                let ss: AuroraErrorInfo =
                    Error::CurrentLoginUserTenantNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::RevokeProjectError(data) => {
                let ss: AuroraErrorInfo = Error::RevokeProjectError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedUser(data) => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedUser(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ProjectNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceHostIsNull(data) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceHostIsNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryExecutingWorkflowError(data) => {
                let ss: AuroraErrorInfo = Error::QueryExecutingWorkflowError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfFunctionNotExist(data) => {
                let ss: AuroraErrorInfo = Error::UdfFunctionNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfFunctionExists(data) => {
                let ss: AuroraErrorInfo = Error::UdfFunctionExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ResourceNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceExist(data) => {
                let ss: AuroraErrorInfo = Error::ResourceExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceSuffixNotSupportView(data) => {
                let ss: AuroraErrorInfo = Error::ResourceSuffixNotSupportView(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceSizeExceedLimit(data) => {
                let ss: AuroraErrorInfo = Error::ResourceSizeExceedLimit(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceSuffixForbidChange(data) => {
                let ss: AuroraErrorInfo = Error::ResourceSuffixForbidChange(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfResourceSuffixNotJar(data) => {
                let ss: AuroraErrorInfo = Error::UdfResourceSuffixNotJar(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::HdfsCopyFail(data) => {
                let ss: AuroraErrorInfo = Error::HdfsCopyFail(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileExist(data) => {
                let ss: AuroraErrorInfo = Error::ResourceFileExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ResourceFileNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfResourceIsBound(data) => {
                let ss: AuroraErrorInfo = Error::UdfResourceIsBound(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceIsUsed(data) => {
                let ss: AuroraErrorInfo = Error::ResourceIsUsed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ParentResourceNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ParentResourceNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceNotExistOrNoPermission(data) => {
                let ss: AuroraErrorInfo =
                    Error::ResourceNotExistOrNoPermission(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceIsAuthorized(data) => {
                let ss: AuroraErrorInfo = Error::ResourceIsAuthorized(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNoOperationPerm(data) => {
                let ss: AuroraErrorInfo = Error::UserNoOperationPerm(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNoOperationProjectPerm(data) => {
                let ss: AuroraErrorInfo = Error::UserNoOperationProjectPerm(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceExist(data) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotRelease(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotRelease(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SubProcessDefineNotRelease(data) => {
                let ss: AuroraErrorInfo = Error::SubProcessDefineNotRelease(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceAlreadyChanged(data) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceAlreadyChanged(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceStateOperationError(data) => {
                let ss: AuroraErrorInfo =
                    Error::ProcessInstanceStateOperationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SubProcessInstanceNotExist(data) => {
                let ss: AuroraErrorInfo = Error::SubProcessInstanceNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotAllowedEdit(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotAllowedEdit(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceExecutingCommand(data) => {
                let ss: AuroraErrorInfo =
                    Error::ProcessInstanceExecutingCommand(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceNotSubProcessInstance(data) => {
                let ss: AuroraErrorInfo =
                    Error::ProcessInstanceNotSubProcessInstance(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceStateCountError(data) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceStateCountError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CountProcessInstanceStateError(data) => {
                let ss: AuroraErrorInfo =
                    Error::CountProcessInstanceStateError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CountProcessDefinitionUserError(data) => {
                let ss: AuroraErrorInfo =
                    Error::CountProcessDefinitionUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StartProcessInstanceError(data) => {
                let ss: AuroraErrorInfo = Error::StartProcessInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchStartProcessInstanceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::BatchStartProcessInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceError(data) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ExecuteProcessInstanceError(data) => {
                let ss: AuroraErrorInfo = Error::ExecuteProcessInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CheckProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::CheckProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRecipientsAndCopyersByProcessDefinitionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryRecipientsAndCopyersByProcessDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DataIsNotValid(data) => {
                let ss: AuroraErrorInfo = Error::DataIsNotValid(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DataIsNull(data) => {
                let ss: AuroraErrorInfo = Error::DataIsNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessNodeHasCycle(data) => {
                let ss: AuroraErrorInfo = Error::ProcessNodeHasCycle(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessNodeSParameterInvalid(data) => {
                let ss: AuroraErrorInfo = Error::ProcessNodeSParameterInvalid(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineStateOnline(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineStateOnline(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefineByCodeError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProcessDefineByCodeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronStateOnline(data) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronStateOnline(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteScheduleCronByIdError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteScheduleCronByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessDefineError(data) => {
                let ss: AuroraErrorInfo = Error::BatchDeleteProcessDefineError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessDefineByCodesError(data) => {
                let ss: AuroraErrorInfo =
                    Error::BatchDeleteProcessDefineByCodesError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefineByCodesError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProcessDefineByCodesError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TenantNotSuitable(data) => {
                let ss: AuroraErrorInfo = Error::TenantNotSuitable(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ExportProcessDefineByIdError(data) => {
                let ss: AuroraErrorInfo = Error::ExportProcessDefineByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchExportProcessDefineByIdsError(data) => {
                let ss: AuroraErrorInfo =
                    Error::BatchExportProcessDefineByIdsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ImportProcessDefineError(data) => {
                let ss: AuroraErrorInfo = Error::ImportProcessDefineError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskDefineNotExist(data) => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateProcessTaskRelationError(data) => {
                let ss: AuroraErrorInfo =
                    Error::CreateProcessTaskRelationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessTaskRelationNotExist(data) => {
                let ss: AuroraErrorInfo = Error::ProcessTaskRelationNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessTaskRelationExist(data) => {
                let ss: AuroraErrorInfo = Error::ProcessTaskRelationExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDagIsEmpty(data) => {
                let ss: AuroraErrorInfo = Error::ProcessDagIsEmpty(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CheckProcessTaskRelationError(data) => {
                let ss: AuroraErrorInfo = Error::CheckProcessTaskRelationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateTaskDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::CreateTaskDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateTaskDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateTaskDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskDefinitionVersionsError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryTaskDefinitionVersionsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SwitchTaskDefinitionVersionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::SwitchTaskDefinitionVersionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskDefinitionVersionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteTaskDefinitionVersionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskDefineByCodeError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteTaskDefineByCodeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDetailOfTaskDefinitionError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryDetailOfTaskDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskDefinitionListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryTaskDefinitionListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskDefinitionNameExisted(data) => {
                let ss: AuroraErrorInfo = Error::TaskDefinitionNameExisted(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ReleaseTaskDefinitionError(data) => {
                let ss: AuroraErrorInfo = Error::ReleaseTaskDefinitionError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MoveProcessTaskRelationError(data) => {
                let ss: AuroraErrorInfo = Error::MoveProcessTaskRelationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskProcessRelationError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteTaskProcessRelationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskProcessRelationError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTaskProcessRelationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskDefineStateOnline(data) => {
                let ss: AuroraErrorInfo = Error::TaskDefineStateOnline(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskHasDownstream(data) => {
                let ss: AuroraErrorInfo = Error::TaskHasDownstream(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskHasUpstream(data) => {
                let ss: AuroraErrorInfo = Error::TaskHasUpstream(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MainTableUsingVersion(data) => {
                let ss: AuroraErrorInfo = Error::MainTableUsingVersion(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectProcessNotMatch(data) => {
                let ss: AuroraErrorInfo = Error::ProjectProcessNotMatch(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteEdgeError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteEdgeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotSupportUpdateTaskDefinition(data) => {
                let ss: AuroraErrorInfo =
                    Error::NotSupportUpdateTaskDefinition(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotSupportCopyTaskType(data) => {
                let ss: AuroraErrorInfo = Error::NotSupportCopyTaskType(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::HdfsNotStartup(data) => {
                let ss: AuroraErrorInfo = Error::HdfsNotStartup(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StorageNotStartup(data) => {
                let ss: AuroraErrorInfo = Error::StorageNotStartup(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::S3CannotRename(data) => {
                let ss: AuroraErrorInfo = Error::S3CannotRename(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDatabaseStateError(data) => {
                let ss: AuroraErrorInfo = Error::QueryDatabaseStateError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateAccessTokenError(data) => {
                let ss: AuroraErrorInfo = Error::CreateAccessTokenError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GenerateTokenError(data) => {
                let ss: AuroraErrorInfo = Error::GenerateTokenError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAccesstokenListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryAccesstokenListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateAccessTokenError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateAccessTokenError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAccessTokenError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteAccessTokenError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AccessTokenNotExist(data) => {
                let ss: AuroraErrorInfo = Error::AccessTokenNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAccesstokenByUserError(data) => {
                let ss: AuroraErrorInfo = Error::QueryAccesstokenByUserError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CommandStateCountError(data) => {
                let ss: AuroraErrorInfo = Error::CommandStateCountError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NegativeSizeNumberError(data) => {
                let ss: AuroraErrorInfo = Error::NegativeSizeNumberError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StartTimeBiggerThanEndTimeError(data) => {
                let ss: AuroraErrorInfo =
                    Error::StartTimeBiggerThanEndTimeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueCountError(data) => {
                let ss: AuroraErrorInfo = Error::QueueCountError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::KerberosStartupState(data) => {
                let ss: AuroraErrorInfo = Error::KerberosStartupState(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuditLogListPaging(data) => {
                let ss: AuroraErrorInfo = Error::QueryAuditLogListPaging(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PluginNotAUiComponent(data) => {
                let ss: AuroraErrorInfo = Error::PluginNotAUiComponent(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginsResultIsNull(data) => {
                let ss: AuroraErrorInfo = Error::QueryPluginsResultIsNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginsError(data) => {
                let ss: AuroraErrorInfo = Error::QueryPluginsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginDetailResultIsNull(data) => {
                let ss: AuroraErrorInfo = Error::QueryPluginDetailResultIsNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateAlertPluginInstanceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::UpdateAlertPluginInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertPluginInstanceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteAlertPluginInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetAlertPluginInstanceError(data) => {
                let ss: AuroraErrorInfo = Error::GetAlertPluginInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateAlertPluginInstanceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::CreateAlertPluginInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAllAlertPluginInstanceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryAllAlertPluginInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PluginInstanceAlreadyExit(data) => {
                let ss: AuroraErrorInfo = Error::PluginInstanceAlreadyExit(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListPagingAlertPluginInstanceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::ListPagingAlertPluginInstanceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(data.clone())
                        .into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionVersionIsUsed(data) => {
                let ss: AuroraErrorInfo =
                    Error::ProcessDefinitionVersionIsUsed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateEnvironmentError(data) => {
                let ss: AuroraErrorInfo = Error::CreateEnvironmentError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentNameExists(data) => {
                let ss: AuroraErrorInfo = Error::EnvironmentNameExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentNameIsNull(data) => {
                let ss: AuroraErrorInfo = Error::EnvironmentNameIsNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentConfigIsNull(data) => {
                let ss: AuroraErrorInfo = Error::EnvironmentConfigIsNull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateEnvironmentError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateEnvironmentError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteEnvironmentError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteEnvironmentError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteEnvironmentRelatedTaskExists(data) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteEnvironmentRelatedTaskExists(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentByNameError(data) => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentByNameError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentByCodeError(data) => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentByCodeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentError(data) => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyEnvironmentError(data) => {
                let ss: AuroraErrorInfo = Error::VerifyEnvironmentError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetRuleFormCreateJsonError(data) => {
                let ss: AuroraErrorInfo = Error::GetRuleFormCreateJsonError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleListPagingError(data) => {
                let ss: AuroraErrorInfo = Error::QueryRuleListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryRuleListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleInputEntryListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryRuleInputEntryListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryExecuteResultListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryExecuteResultListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceOptionsError(data) => {
                let ss: AuroraErrorInfo = Error::GetDatasourceOptionsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceTablesError(data) => {
                let ss: AuroraErrorInfo = Error::GetDatasourceTablesError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceTableColumnsError(data) => {
                let ss: AuroraErrorInfo =
                    Error::GetDatasourceTableColumnsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupNameExist(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupNameExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupSizeError(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupSizeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusError(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupFull(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupFull(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupUsedSizeError(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupUsedSizeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueReleaseError(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueReleaseError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueAwakeError(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueAwakeError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateTaskGroupError(data) => {
                let ss: AuroraErrorInfo = Error::CreateTaskGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateTaskGroupError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateTaskGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskGroupListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTaskGroupListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CloseTaskGroupError(data) => {
                let ss: AuroraErrorInfo = Error::CloseTaskGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StartTaskGroupError(data) => {
                let ss: AuroraErrorInfo = Error::StartTaskGroupError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskGroupQueueListError(data) => {
                let ss: AuroraErrorInfo = Error::QueryTaskGroupQueueListError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupCacheStartFailed(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupCacheStartFailed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentWorkerGroupsIsInvalid(data) => {
                let ss: AuroraErrorInfo =
                    Error::EnvironmentWorkerGroupsIsInvalid(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateEnvironmentWorkerGroupRelationError(data) => {
                let ss: AuroraErrorInfo =
                    Error::UpdateEnvironmentWorkerGroupRelationError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueAlreadyStart(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueAlreadyStart(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusClosed(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusClosed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusOpened(data) => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusOpened(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotAllowToDisableOwnAccount(data) => {
                let ss: AuroraErrorInfo = Error::NotAllowToDisableOwnAccount(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotAllowToDeleteDefaultAlarmGroup(data) => {
                let ss: AuroraErrorInfo =
                    Error::NotAllowToDeleteDefaultAlarmGroup(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TimeZoneIllegal(data) => {
                let ss: AuroraErrorInfo = Error::TimeZoneIllegal(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryK8sNamespaceListPagingError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryK8sNamespaceListPagingError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::K8sNamespaceExist(data) => {
                let ss: AuroraErrorInfo = Error::K8sNamespaceExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateK8sNamespaceError(data) => {
                let ss: AuroraErrorInfo = Error::CreateK8sNamespaceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateK8sNamespaceError(data) => {
                let ss: AuroraErrorInfo = Error::UpdateK8sNamespaceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::K8sNamespaceNotExist(data) => {
                let ss: AuroraErrorInfo = Error::K8sNamespaceNotExist(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::K8sClientOpsError(data) => {
                let ss: AuroraErrorInfo = Error::K8sClientOpsError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyK8sNamespaceError(data) => {
                let ss: AuroraErrorInfo = Error::VerifyK8sNamespaceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteK8sNamespaceByIdError(data) => {
                let ss: AuroraErrorInfo = Error::DeleteK8sNamespaceByIdError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyParameterNameFailed(data) => {
                let ss: AuroraErrorInfo = Error::VerifyParameterNameFailed(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StoreOperateCreateError(data) => {
                let ss: AuroraErrorInfo = Error::StoreOperateCreateError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantK8sNamespaceError(data) => {
                let ss: AuroraErrorInfo = Error::GrantK8sNamespaceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUnauthorizedNamespaceError(data) => {
                let ss: AuroraErrorInfo =
                    Error::QueryUnauthorizedNamespaceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedNamespaceError(data) => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedNamespaceError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryCanUseK8sClusterError(data) => {
                let ss: AuroraErrorInfo = Error::QueryCanUseK8sClusterError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFullNameTooLongError(data) => {
                let ss: AuroraErrorInfo = Error::ResourceFullNameTooLongError(data.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TenantFullNameTooLongError(data) => {
                let ss: AuroraErrorInfo = Error::TenantFullNameTooLongError(data.clone()).into();
                write!(f, "{}", ss)
            }
        }
    }
}

impl std::error::Error for Error {}
pub type AuroraData = serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuroraErrorInfo {
    pub code: i32,
    // #[cfg(feature = "en_msg")]
    pub en_msg: String,
    // #[cfg(feature = "cn_msg")]
    pub cn_msg: String,
    pub error_data: AuroraData,
}

impl Default for AuroraErrorInfo {
    fn default() -> Self {
        Self {
            code: 0,
            en_msg: "success".to_string(),
            cn_msg: "成功".to_string(),
            error_data: AuroraData::Null,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DisplayErrorInfo {
    pub code: i32,
    // #[cfg(feature = "en_msg")]
    pub msg: String,
}
#[cfg(feature = "cn_msg")]
impl From<AuroraErrorInfo> for DisplayErrorInfo {
    fn from(value: AuroraErrorInfo) -> Self {
        Self {
            code: value.code,
            msg: value.cn_msg,
        }
    }
}
#[cfg(feature = "en_msg")]
impl From<AuroraErrorInfo> for DisplayErrorInfo {
    fn from(value: AuroraErrorInfo) -> Self {
        Self {
            code: value.code,
            msg: value.en_msg,
        }
    }
}

impl AuroraErrorInfo {
    pub fn new(code: i32, en_msg: String, cn_msg: String) -> AuroraErrorInfo {
        AuroraErrorInfo {
            code,
            en_msg,
            cn_msg,
            error_data: AuroraData::Null,
        }
    }
    pub fn new_with_data(&mut self, error_data: AuroraData) -> AuroraErrorInfo {
        AuroraErrorInfo {
            error_data,
            ..self.clone()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ParseAuroraErrorInfoError;

impl FromStr for AuroraErrorInfo {
    type Err = ParseAuroraErrorInfoError;

    fn from_str(msg: &str) -> Result<Self, Self::Err> {
        let code_info: Vec<_> = msg.split('~').collect();
        let code = &code_info[0]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"10000")
            .parse::<i32>()
            .unwrap_or(10000);
        let en_msg = code_info[1]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"internal server error please check the log")
            .to_string();
        let cn_msg = code_info[2]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"内部服务错误，请查看日志")
            .to_string();

        let err_data = code_info[3]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .map(|x| serde_json::from_str::<AuroraData>(x).unwrap_or_default());
        Ok(AuroraErrorInfo {
            code: *code,
            en_msg,
            cn_msg,
            error_data: err_data.unwrap_or_default(),
        })
    }
}

impl From<String> for AuroraErrorInfo {
    fn from(msg: String) -> Self {
        let code_info: Vec<_> = msg.split('~').collect();
        let code = &code_info[0]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"10000")
            .parse::<i32>()
            .unwrap_or(10000);
        let en_msg = code_info[1]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"internal server error please check the log")
            .to_string();
        let cn_msg = code_info[2]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"内部服务错误，请查看日志")
            .to_string();
        let err_data = code_info[3]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .map(|x| serde_json::from_str::<AuroraData>(x).unwrap_or_default());
        AuroraErrorInfo {
            code: *code,
            en_msg,
            cn_msg,
            error_data: err_data.unwrap_or_default(),
        }
    }
}

impl std::error::Error for AuroraErrorInfo {}
impl core::fmt::Display for AuroraErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "code|{}~en_msg|{}~cn_msg|{}~error_data|{}",
            self.code,
            self.en_msg,
            self.cn_msg,
            json!(self.error_data)
        )
    }
}

impl From<AuroraErrorInfo> for Error {
    fn from(value: AuroraErrorInfo) -> Self {
        match (
            value.code,
            value.en_msg.as_str(),
            value.cn_msg.as_str(),
            value.error_data.clone(),
        ) {
            (0, ..) => Error::SUCCESS(value.error_data),
            (10000, ..) => Error::InternalServerErrorArgs(value.error_data),
            (10001, ..) => Error::RequestParamsNotValidError(value.error_data),
            (10002, ..) => Error::TaskTimeoutParamsError(value.error_data),
            (10003, ..) => Error::UserNameExist(value.error_data),
            (10004, ..) => Error::UserNameNull(value.error_data),
            (10006, ..) => Error::HdfsOperationError(value.error_data),
            (10018, ..) => Error::ProjectNotFound(value.error_data),
            (10019, ..) => Error::ProjectAlreadyExists(value.error_data),
            (10020, ..) => Error::TaskInstanceNotExists(value.error_data),
            (10030, ..) => Error::UpdateAlertGroupError(value.error_data), //(10030, "update alert group error", "更新告警组错误"),
            (10031, ..) => Error::DeleteAlertGroupError(value.error_data), //(10031, "delete alert group error", "删除告警组错误"),
            (10032, ..) => Error::AlertGroupGrantUserError(value.error_data), //(10032, "alert group grant user error", "告警组授权用户错误"),
            (10033, ..) => Error::CreateDatasourceError(value.error_data), //(10033, "create datasource error", "创建数据源错误"),
            (10034, ..) => Error::UpdateDatasourceError(value.error_data), //(10034, "update datasource error", "更新数据源错误"),
            (10035, ..) => Error::QueryDatasourceError(value.error_data), //(10035, "query datasource error", "查询数据源错误"),
            (10036, ..) => Error::ConnectDatasourceFailure(value.error_data), //(10036, "connect datasource failure", "建立数据源连接失败"),
            (10037, ..) => Error::ConnectionTestFailure(value.error_data), //(10037, "connection test failure", "测试数据源连接失败"),
            (10038, ..) => Error::DeleteDataSourceFailure(value.error_data), //(10038, "delete data source failure", "删除数据源失败"),
            (10039, ..) => Error::VerifyDatasourceNameFailure(value.error_data), //(10039, "verify datasource name failure", "验证数据源名称失败"),
            (10040, ..) => Error::UnauthorizedDatasource(value.error_data), //(10040, "unauthorized datasource", "未经授权的数据源"),
            (10041, ..) => Error::AuthorizedDataSource(value.error_data), //(10041, "authorized data source", "授权数据源失败"),
            (10042, ..) => Error::LoginSuccess(value.error_data), //(10042, "login success", "登录成功"),
            (10043, ..) => Error::UserLoginFailure(value.error_data), //(10043, "user login failure", "用户登录失败"),
            (10044, ..) => Error::ListWorkersError(value.error_data), //(10044, "list workers error", "查询worker列表错误"),
            (10045, ..) => Error::ListMastersError(value.error_data), //(10045, "list masters error", "查询master列表错误"),
            (10046, ..) => Error::UpdateProjectError(value.error_data), //(10046, "update project error", "更新项目信息错误"),
            (10047, ..) => Error::QueryProjectDetailsByCodeError(value.error_data), //(10047, "query project details by code error", "查询项目详细信息错误"),
            (10048, ..) => Error::CreateProjectError(value.error_data), //(10048, "create project error", "创建项目错误"),
            (10049, ..) => Error::LoginUserQueryProjectListPagingError(value.error_data), //(10049, "login user query project list paging error", "分页查询项目列表错误"),
            (10050, ..) => Error::DeleteProjectError(value.error_data), //(10050, "delete project error", "删除项目错误"),
            (10051, ..) => Error::QueryUnauthorizedProjectError(value.error_data), //(10051, "query unauthorized project error", "查询未授权项目错误"),
            (10052, ..) => Error::QueryAuthorizedProject(value.error_data), //(10052, "query authorized project", "查询授权项目错误"),
            (10053, ..) => Error::QueryQueueListError(value.error_data), //(10053, "query queue list error", "查询队列列表错误"),
            (10054, ..) => Error::CreateResourceError(value.error_data), //(10054, "create resource error", "创建资源错误"),
            (10055, ..) => Error::UpdateResourceError(value.error_data), //(10055, "update resource error", "更新资源错误"),
            (10056, ..) => Error::QueryResourcesListError(value.error_data), //(10056, "query resources list error", "查询资源列表错误"),
            (10057, "query resources list paging", "分页查询资源列表错误", ..) => {
                Error::QueryResourcesListPaging(value.error_data)
            }
            //(10057, "query resources list paging", "分页查询资源列表错误"),
            (10058, ..) => Error::DeleteResourceError(value.error_data), //(10058, "delete resource error", "删除资源错误"),
            (10059, ..) => Error::VerifyResourceByNameAndTypeError(value.error_data), //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
            (10060, ..) => Error::ViewResourceFileOnLineError(value.error_data), //(10060, "view resource file online error", "查看资源文件错误"),
            (10061, ..) => Error::CreateResourceFileOnLineError(value.error_data), //(10061, "create resource file online error", "创建资源文件错误"),
            (10062, ..) => Error::ResourceFileIsEmpty(value.error_data), //(10062, "resource file is empty", "资源文件内容不能为空"),
            (10063, ..) => Error::EditResourceFileOnLineError(value.error_data), //(10063, "edit resource file online error", "更新资源文件错误"),
            (10064, ..) => Error::DownloadResourceFileError(value.error_data), //(10064, "download resource file error", "下载资源文件错误"),
            (10065, ..) => Error::CreateUdfFunctionError(value.error_data), //(10065, "create udf function error", "创建UDF函数错误"),
            (10066, ..) => Error::ViewUdfFunctionError(value.error_data), //(10066, "view udf function error", "查询UDF函数错误"),
            (10067, ..) => Error::UpdateUdfFunctionError(value.error_data), //(10067, "update udf function error", "更新UDF函数错误"),
            (10068, ..) => Error::QueryUdfFunctionListPagingError(value.error_data), //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
            (10069, ..) => Error::QueryDatasourceByTypeError(value.error_data), //(10069, "query datasource by type error", "查询数据源信息错误"),
            (10070, ..) => Error::VerifyUdfFunctionNameError(value.error_data), //(10070, "verify udf function name error", "UDF函数名称验证错误"),
            (10071, ..) => Error::DeleteUdfFunctionError(value.error_data), //(10071, "delete udf function error", "删除UDF函数错误"),
            (10072, ..) => Error::AuthorizedFileResourceError(value.error_data), //(10072, "authorized file resource error", "授权资源文件错误"),
            (10073, ..) => Error::AuthorizeResourceTree(value.error_data), //(10073, "authorize resource tree display error", "授权资源目录树错误"),
            (10074, ..) => Error::UnauthorizedUdfFunctionError(value.error_data), //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
            (10075, ..) => Error::AuthorizedUdfFunctionError(value.error_data), //(10075, "authorized udf function error", "授权UDF函数错误"),
            (10076, ..) => Error::CreateScheduleError(value.error_data), //(10076, "create schedule error", "创建调度配置错误"),
            (10077, ..) => Error::UpdateScheduleError(value.error_data), //(10077, "update schedule error", "更新调度配置错误"),
            (10078, ..) => Error::PublishScheduleOnlineError(value.error_data), //(10078, "publish schedule online error", "上线调度配置错误"),
            (10079, ..) => Error::OfflineScheduleError(value.error_data), //(10079, "offline schedule error", "下线调度配置错误"),
            (10080, ..) => Error::QueryScheduleListPagingError(value.error_data), //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
            (10081, ..) => Error::QueryScheduleListError(value.error_data), //(10081, "query schedule list error", "查询调度配置列表错误"),
            (10082, ..) => Error::QueryTaskListPagingError(value.error_data), //(10082, "query task list paging error", "分页查询任务列表错误"),
            (10083, ..) => Error::QueryTaskRecordListPagingError(value.error_data), //(10083, "query task record list paging error", "分页查询任务记录错误"),
            (10084, ..) => Error::CreateTenantError(value.error_data), //(10084, "create tenant error", "创建租户错误"),
            (10085, ..) => Error::QueryTenantListPagingError(value.error_data), //(10085, "query tenant list paging error", "分页查询租户列表错误"),
            (10086, ..) => Error::QueryTenantListError(value.error_data), //(10086, "query tenant list error", "查询租户列表错误"),
            (10087, ..) => Error::UpdateTenantError(value.error_data), //(10087, "update tenant error", "更新租户错误"),
            (10088, ..) => Error::DeleteTenantByIdError(value.error_data), //(10088, "delete tenant by id error", "删除租户错误"),
            (10089, ..) => Error::VerifyOsTenantCodeError(value.error_data), //(10089, "verify os tenant code error", "操作系统租户验证错误"),
            (10090, ..) => Error::CreateUserError(value.error_data), //(10090, "create user error", "创建用户错误"),
            (10091, ..) => Error::QueryUserListPagingError(value.error_data), //(10091, "query user list paging error", "分页查询用户列表错误"),
            (10092, ..) => Error::UpdateUserError(value.error_data), //(10092, "update user error", "更新用户错误"),
            (10093, ..) => Error::DeleteUserByIdError(value.error_data), //(10093, "delete user by id error", "删除用户错误"),
            (10094, ..) => Error::GrantProjectError(value.error_data), //(10094, "grant project error", "授权项目错误"),
            (10095, ..) => Error::GrantResourceError(value.error_data), //(10095, "grant resource error", "授权资源错误"),
            (10096, ..) => Error::GrantUdfFunctionError(value.error_data), //(10096, "grant udf function error", "授权UDF函数错误"),
            (10097, ..) => Error::GrantDatasourceError(value.error_data), //(10097, "grant datasource error", "授权数据源错误"),
            (10098, ..) => Error::GetUserInfoError(value.error_data), //(10098, "get user info error", "获取用户信息错误"),
            (10099, ..) => Error::UserListError(value.error_data), //(10099, "user list error", "查询用户列表错误"),
            (10100, ..) => Error::VerifyUsernameError(value.error_data), //(10100, "verify username error", "用户名验证错误"),
            (10101, ..) => Error::UnauthorizedUserError(value.error_data), //(10101, "unauthorized user error", "查询未授权用户错误"),
            (10102, ..) => Error::AuthorizedUserError(value.error_data), //(10102, "authorized user error", "查询授权用户错误"),
            (10103, ..) => Error::QueryTaskInstanceLogError(value.error_data), //(10103, "view task instance log error", "查询任务实例日志错误"),
            (10104, ..) => Error::DownloadTaskInstanceLogFileError(value.error_data), //(10104, "download task instance log file error", "下载任务日志文件错误"),
            (10105, ..) => Error::CreateProcessDefinitionError(value.error_data), //(10105, "create process definition error", "创建工作流错误"),
            (10106, ..) => Error::VerifyProcessDefinitionNameUniqueError(value.error_data), //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
            (10107, ..) => Error::UpdateProcessDefinitionError(value.error_data), //(10107, "update process definition error", "更新工作流定义错误"),
            (10108, ..) => Error::ReleaseProcessDefinitionError(value.error_data), //(10108, "release process definition error", "上线工作流错误"),
            (10109, ..) => Error::QueryDetailOfProcessDefinitionError(value.error_data), //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
            (10110, ..) => Error::QueryProcessDefinitionList(value.error_data), //(10110, "query process definition list", "查询工作流列表错误"),
            (10111, ..) => Error::EncapsulationTreeviewStructureError(value.error_data), //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
            (10112, ..) => Error::GetTasksListByProcessDefinitionIdError(value.error_data), //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
            (10113, ..) => Error::QueryProcessInstanceListPagingError(value.error_data), //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
            (10114, ..) => Error::QueryTaskListByProcessInstanceIdError(value.error_data), //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
            (10115, ..) => Error::UpdateProcessInstanceError(value.error_data), //(10115, "update process instance error", "更新工作流实例错误"),
            (10116, ..) => Error::QueryProcessInstanceByIdError(value.error_data), //(10116, "query process instance by id error", "查询工作流实例错误"),
            (10117, "delete process instance by id error", "删除工作流实例错误", ..) => {
                Error::DeleteProcessInstanceByIdError(value.error_data)
            } //(10117, "delete process instance by id error", "删除工作流实例错误"),
            (10118, ..) => Error::QuerySubProcessInstanceDetailInfoByTaskIdError(value.error_data), //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
            (10119, ..) => Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(
                value.error_data,
            ), //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
            (10120, ..) => Error::QueryProcessInstanceAllVariablesError(value.error_data), //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
            (10121, ..) => Error::EncapsulationProcessInstanceGanttStructureError(value.error_data), //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
            (10122, ..) => Error::QueryProcessDefinitionListPagingError(value.error_data), //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
            (10123, ..) => Error::SignOutError(value.error_data), //(10123, "sign out error", "退出错误"),
            (10124, ..) => Error::OsTenantCodeHasAlreadyExists(value.error_data), //(10124, "os tenant code has already exists", "操作系统租户已存在"),
            (10125, ..) => Error::IpIsEmpty(value.error_data), //(10125, "ip is empty", "IP地址不能为空"),
            (10126, ..) => Error::ScheduleCronReleaseNeedNotChange(value.error_data), //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
            (10127, ..) => Error::CreateQueueError(value.error_data), //(10127, "create queue error", "创建队列错误"),
            (10128, ..) => Error::QueueNotExist(value.error_data), //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
            (10129, ..) => Error::QueueValueExist(value.error_data), //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
            (10130, ..) => Error::QueueNameExist(value.error_data), //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
            (10131, ..) => Error::UpdateQueueError(value.error_data), //(10131, "update queue error", "更新队列信息错误"),
            (10132, ..) => Error::NeedNotUpdateQueue(value.error_data), //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
            (10133, ..) => Error::VerifyQueueError(value.error_data), //(10133, "verify queue error", "验证队列信息错误"),
            (10134, ..) => Error::NameNull(value.error_data), //(10134, "name must be not null", "名称不能为空"),
            (10135, ..) => Error::NameExist(value.error_data), //(10135, "name {0} already exists", "名称[{0}]已存在"),
            (10136, ..) => Error::SaveError(value.error_data), //(10136, "save error", "保存错误"),
            (
                10117,
                "please delete the process definitions in project first!",
                "请先删除全部工作流定义",
                ..,
            ) => Error::DeleteProjectErrorDefinesNotNull(value.error_data), //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
            (10138, ..) => Error::BatchDeleteProcessInstanceByIdsError(value.error_data), //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
            (10139, ..) => Error::PreviewScheduleError(value.error_data), //(10139, "preview schedule error", "预览调度配置错误"),
            (10140, ..) => Error::ParseToCronExpressionError(value.error_data), //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
            (10141, ..) => Error::ScheduleStartTimeEndTimeSame(value.error_data), //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
            (10142, ..) => Error::DeleteTenantByIdFail(value.error_data), //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
            (10143, ..) => Error::DeleteTenantByIdFailDefines(value.error_data), //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
            (10144, ..) => Error::DeleteTenantByIdFailUsers(value.error_data), //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
            (10145, ..) => Error::DeleteWorkerGroupByIdFail(value.error_data), //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
            (10146, ..) => Error::QueryWorkerGroupFail(value.error_data), //(10146, "query worker group fail ", "查询worker分组失败"),
            (10147, ..) => Error::DeleteWorkerGroupFail(value.error_data), //(10147, "delete worker group fail ", "删除worker分组失败"),
            (10148, ..) => Error::UserDisabled(value.error_data), //(10148, "The current user is disabled", "当前用户已停用"),
            (10149, ..) => Error::CopyProcessDefinitionError(value.error_data), //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
            (10150, ..) => Error::MoveProcessDefinitionError(value.error_data), //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
            (10151, ..) => Error::SwitchProcessDefinitionVersionError(value.error_data), //(10151, "Switch process definition version error", "切换工作流版本出错"),
            (10152, ..) => Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(
                value.error_data,
            ), //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
            (10153, ..) => {
                Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(
                    value.error_data,
                )
            } //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
            (10154, ..) => Error::QueryProcessDefinitionVersionsError(value.error_data), //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
            (10156, ..) => Error::DeleteProcessDefinitionVersionError(value.error_data), //(10156, "delete process definition version error", "删除工作流历史版本出错"),

            (10157, ..) => Error::QueryUserCreatedProjectError(value.error_data), //(10157, "query user created project error error", "查询用户创建的项目错误"),
            (10158, ..) => Error::ProcessDefinitionCodesIsEmpty(value.error_data), //(10158, "process definition codes is empty", "工作流CODES不能为空"),
            (10159, ..) => Error::BatchCopyProcessDefinitionError(value.error_data), //(10159, "batch copy process definition error", "复制工作流错误"),
            (10160, ..) => Error::BatchMoveProcessDefinitionError(value.error_data), //(10160, "batch move process definition error", "移动工作流错误"),
            (10161, ..) => Error::QueryWorkflowLineageError(value.error_data), //(10161, "query workflow lineage error", "查询血缘失败"),
            (10162, ..) => Error::QueryAuthorizedAndUserCreatedProjectError(value.error_data), //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
            (10163, ..) => Error::DeleteProcessDefinitionByCodeFail(value.error_data), //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
            (10164, ..) => Error::CheckOsTenantCodeError(value.error_data), //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
            (10165, ..) => Error::ForceTaskSuccessError(value.error_data), //(10165, "force task success error", "强制成功任务实例错误"),
            (10166, ..) => Error::TaskInstanceStateOperationError(value.error_data), //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
            (10167, ..) => Error::DatasourceTypeNotExist(value.error_data), //(10167, "data source type not exist", "数据源类型不存在"),
            (10168, ..) => Error::ProcessDefinitionNameExist(value.error_data), //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
            (10169, ..) => Error::DatasourceDbTypeIllegal(value.error_data), //(10169, "datasource type illegal", "数据源类型参数不合法"),
            (10170, ..) => Error::DatasourcePortIllegal(value.error_data), //(10170, "datasource port illegal", "数据源端口参数不合法"),
            (10171, ..) => Error::DatasourceOtherParamsIllegal(value.error_data), //(10171, "datasource other params illegal", "数据源其他参数不合法"),
            (10172, ..) => Error::DatasourceNameIllegal(value.error_data), //(10172, "datasource name illegal", "数据源名称不合法"),
            (10173, ..) => Error::DatasourceHostIllegal(value.error_data), //(10173, "datasource host illegal", "数据源HOST不合法"),
            (10174, ..) => Error::DeleteWorkerGroupNotExist(value.error_data), //(10174, "delete worker group not exist ", "删除worker分组不存在"),
            (10175, ..) => Error::CreateWorkerGroupForbiddenInDocker(value.error_data), //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
            (10176, ..) => Error::DeleteWorkerGroupForbiddenInDocker(value.error_data), //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
            (10177, ..) => Error::WorkerAddressInvalid(value.error_data), //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
            (10178, ..) => Error::QueryWorkerAddressListFail(value.error_data), //(10178, "query worker address list fail ", "查询worker地址列表失败"),
            (10179, ..) => Error::TransformProjectOwnership(value.error_data), //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
            (10180, ..) => Error::QueryAlertGroupError(value.error_data), //(10180, "query alert group error", "查询告警组错误"),
            (10181, ..) => Error::CurrentLoginUserTenantNotExist(value.error_data), //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
            (10182, ..) => Error::RevokeProjectError(value.error_data), //(10182, "revoke project error", "撤销项目授权错误"),
            (10183, ..) => Error::QueryAuthorizedUser(value.error_data), //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
            (10184, ..) => Error::ProjectNotExist(value.error_data), //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
            (10185, ..) => Error::TaskInstanceHostIsNull(value.error_data), //(10191, "task instance host is null", "任务实例host为空"),
            (10186, ..) => Error::QueryExecutingWorkflowError(value.error_data), //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

            (20001, ..) => Error::UdfFunctionNotExist(value.error_data), //(20001, "UDF function not found", "UDF函数不存在"),
            (20002, ..) => Error::UdfFunctionExists(value.error_data), //(20002, "UDF function already exists", "UDF函数已存在"),
            (20004, ..) => Error::ResourceNotExist(value.error_data), //(20004, "resource not exist", "资源不存在"),
            (20005, ..) => Error::ResourceExist(value.error_data), //(20005, "resource already exists", "资源已存在"),
            (20006, ..) => Error::ResourceSuffixNotSupportView(value.error_data), //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
            (20007, ..) => Error::ResourceSizeExceedLimit(value.error_data), //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
            (20008, ..) => Error::ResourceSuffixForbidChange(value.error_data), //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
            (20009, ..) => Error::UdfResourceSuffixNotJar(value.error_data), //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
            (20010, ..) => Error::HdfsCopyFail(value.error_data), //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
            (20011, ..) => Error::ResourceFileExist(value.error_data), //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
            (20012, ..) => Error::ResourceFileNotExist(value.error_data), //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
            (20013, ..) => Error::UdfResourceIsBound(value.error_data), //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
            (20014, ..) => Error::ResourceIsUsed(value.error_data), //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
            (20015, ..) => Error::ParentResourceNotExist(value.error_data), //(20015, "parent resource not exist", "父资源文件不存在"),
            (20016, ..) => Error::ResourceNotExistOrNoPermission(value.error_data), //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
            (20017, ..) => Error::ResourceIsAuthorized(value.error_data), //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

            (30001, ..) => Error::UserNoOperationPerm(value.error_data), //(30001, "user has no operation privilege", "当前用户没有操作权限"),
            (30002, ..) => Error::UserNoOperationProjectPerm(value.error_data), //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

            (50001, ..) => Error::ProcessInstanceNotExist(value.error_data), //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
            (50002, ..) => Error::ProcessInstanceExist(value.error_data), //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
            (50003, ..) => Error::ProcessDefineNotExist(value.error_data), //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
            (
                50004,
                "process definition {0} process version {1} not online",
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态",
                ..,
            ) => Error::ProcessDefineNotRelease(value.error_data), //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
            (
                50004,
                "exist sub process definition not online",
                "存在子工作流定义不是上线状态",
                ..,
            ) => Error::SubProcessDefineNotRelease(value.error_data), //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
            (50005, ..) => Error::ProcessInstanceAlreadyChanged(value.error_data), //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
            (50006, ..) => Error::ProcessInstanceStateOperationError(value.error_data), //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
            (50007, ..) => Error::SubProcessInstanceNotExist(value.error_data), //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
            (50008, ..) => Error::ProcessDefineNotAllowedEdit(value.error_data), //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
            (50009, ..) => Error::ProcessInstanceExecutingCommand(value.error_data), //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
            (50010, ..) => Error::ProcessInstanceNotSubProcessInstance(value.error_data), //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
            (50011, ..) => Error::TaskInstanceStateCountError(value.error_data), //(50011, "task instance state count error", "查询各状态任务实例数错误"),
            (50012, ..) => Error::CountProcessInstanceStateError(value.error_data), //(50012, "count process instance state error", "查询各状态流程实例数错误"),
            (50013, ..) => Error::CountProcessDefinitionUserError(value.error_data), //(50013, "count process definition user error", "查询各用户流程定义数错误"),
            (50014, "start process instance error", "运行工作流实例错误", ..) => {
                Error::StartProcessInstanceError(value.error_data)
            } //(50014, "start process instance error", "运行工作流实例错误"),
            (
                50014,
                "batch start process instance error: {0}",
                "批量运行工作流实例错误: {0}",
                ..,
            ) => Error::BatchStartProcessInstanceError(value.error_data), //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
            (50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误", ..) => {
                Error::ProcessInstanceError(value.error_data)
            } //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
            (50015, "execute process instance error", "操作工作流实例错误", ..) => {
                Error::ExecuteProcessInstanceError(value.error_data)
            } //(50015, "execute process instance error", "操作工作流实例错误")
            (50016, "check process definition error", "工作流定义错误", ..) => {
                Error::CheckProcessDefinitionError(value.error_data)
            } //(50016, "check process definition error", "工作流定义错误")
            (
                50017,
                "query recipients and copyers by process definition error",
                "查询收件人和抄送人错误",
                ..,
            ) => Error::QueryRecipientsAndCopyersByProcessDefinitionError(value.error_data), //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误")
            (50017, "data {0} not valid", "数据[{0}]无效", ..) => {
                Error::DataIsNotValid(value.error_data)
            } //(50017, "data {0} not valid", "数据[{0}]无效")
            (50018, "data {0} is null", "数据[{0}]不能为空", ..) => {
                Error::DataIsNull(value.error_data)
            } //(50018, "data {0} is null", "数据[{0}]不能为空")
            (50019, "process node has cycle", "流程节点间存在循环依赖", ..) => {
                Error::ProcessNodeHasCycle(value.error_data)
            } //(50019, "process node has cycle", "流程节点间存在循环依赖")
            (50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效", ..) => {
                Error::ProcessNodeSParameterInvalid(value.error_data)
            } //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效")
            (50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线", ..) => {
                Error::ProcessDefineStateOnline(value.error_data)
            } //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线")
            (50022, "delete process definition by code error", "删除工作流定义错误", ..) => {
                Error::DeleteProcessDefineByCodeError(value.error_data)
            } //(50022, "delete process definition by code error", "删除工作流定义错误")
            (50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线", ..) => {
                Error::ScheduleCronStateOnline(value.error_data)
            } //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线")
            (50024, "delete schedule by id error", "删除调度配置错误", ..) => {
                Error::DeleteScheduleCronByIdError(value.error_data)
            } //(50024, "delete schedule by id error", "删除调度配置错误")
            (50025, "batch delete process definition error", "批量删除工作流定义错误", ..) => {
                Error::BatchDeleteProcessDefineError(value.error_data)
            } //(50025, "batch delete process definition error", "批量删除工作流定义错误")
            (
                50026,
                "batch delete process definition by codes {0} error",
                "批量删除工作流定义[{0}]错误",
                ..,
            ) => Error::BatchDeleteProcessDefineByCodesError(value.error_data), //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误")
            (
                50026,
                "delete process definition by codes {0} error",
                "删除工作流定义[{0}]错误",
                ..,
            ) => Error::DeleteProcessDefineByCodesError(value.error_data), //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误")
            (
                50027,
                "there is not any tenant suitable, please choose a tenant available.",
                "没有合适的租户，请选择可用的租户",
                ..,
            ) => Error::TenantNotSuitable(value.error_data), //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户")
            (50028, "export process definition by id error", "导出工作流定义错误", ..) => {
                Error::ExportProcessDefineByIdError(value.error_data)
            } //(50028, "export process definition by id error", "导出工作流定义错误")
            (
                50028,
                "batch export process definition by ids error",
                "批量导出工作流定义错误",
                ..,
            ) => Error::BatchExportProcessDefineByIdsError(value.error_data), //(50028, "batch export process definition by ids error", "批量导出工作流定义错误")
            (50029, "import process definition error", "导入工作流定义错误", ..) => {
                Error::ImportProcessDefineError(value.error_data)
            } //(50029, "import process definition error", "导入工作流定义错误")
            (50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在", ..) => {
                Error::TaskDefineNotExist(value.error_data)
            } //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在")
            (50032, "create process task relation error", "创建工作流任务关系错误", ..) => {
                Error::CreateProcessTaskRelationError(value.error_data)
            } //(50032, "create process task relation error", "创建工作流任务关系错误")
            (
                50033,
                "process task relation [{0}] does not exist",
                "工作流任务关系[{0}]不存在",
                ..,
            ) => Error::ProcessTaskRelationNotExist(value.error_data), //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在")
            (
                50034,
                "process task relation is already exist, processCode:[{0}]",
                "工作流任务关系已存在, processCode:[{0}]",
                ..,
            ) => Error::ProcessTaskRelationExist(value.error_data), //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]")
            (50035, "process dag is empty", "工作流dag是空", ..) => {
                Error::ProcessDagIsEmpty(value.error_data)
            } //(50035, "process dag is empty", "工作流dag是空")
            (50036, "check process task relation error", "工作流任务关系参数错误", ..) => {
                Error::CheckProcessTaskRelationError(value.error_data)
            } //(50036, "check process task relation error", "工作流任务关系参数错误")
            (50037, "create task definition error", "创建任务错误", ..) => {
                Error::CreateTaskDefinitionError(value.error_data)
            } //(50037, "create task definition error", "创建任务错误")
            (50038, "update task definition error", "更新任务定义错误", ..) => {
                Error::UpdateTaskDefinitionError(value.error_data)
            } //(50038, "update task definition error", "更新任务定义错误")
            (50039, "query task definition versions error", "查询任务历史版本信息出错", ..) => {
                Error::QueryTaskDefinitionVersionsError(value.error_data)
            } //(50039, "query task definition versions error", "查询任务历史版本信息出错")
            (50040, "Switch task definition version error", "切换任务版本出错", ..) => {
                Error::SwitchTaskDefinitionVersionError(value.error_data)
            } //(50040, "Switch task definition version error", "切换任务版本出错")
            (50041, "delete task definition version error", "删除任务历史版本出错", ..) => {
                Error::DeleteTaskDefinitionVersionError(value.error_data)
            } //(50041, "delete task definition version error", "删除任务历史版本出错")
            (50042, "delete task definition by code error", "删除任务定义错误", ..) => {
                Error::DeleteTaskDefineByCodeError(value.error_data)
            } //(50042, "delete task definition by code error", "删除任务定义错误")
            (50043, "query detail of task definition error", "查询任务详细信息错误", ..) => {
                Error::QueryDetailOfTaskDefinitionError(value.error_data)
            } //(50043, "query detail of task definition error", "查询任务详细信息错误")
            (50044, "query task definition list paging error", "分页查询任务定义列表错误", ..) => {
                Error::QueryTaskDefinitionListPagingError(value.error_data)
            } //(50044, "query task definition list paging error", "分页查询任务定义列表错误")
            (
                50045,
                "task definition name [{0}] already exists",
                "任务定义名称[{0}]已经存在",
                ..,
            ) => Error::TaskDefinitionNameExisted(value.error_data), //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在")
            (50046, "release task definition error", "上线任务错误", ..) => {
                Error::ReleaseTaskDefinitionError(value.error_data)
            } //(50046, "release task definition error", "上线任务错误")
            (50047, "move process task relation error", "移动任务到其他工作流错误", ..) => {
                Error::MoveProcessTaskRelationError(value.error_data)
            } //(50047, "move process task relation error", "移动任务到其他工作流错误")
            (50048, "delete process task relation error", "删除工作流任务关系错误", ..) => {
                Error::DeleteTaskProcessRelationError(value.error_data)
            } //(50048, "delete process task relation error", "删除工作流任务关系错误")
            (50049, "query process task relation error", "查询工作流任务关系错误", ..) => {
                Error::QueryTaskProcessRelationError(value.error_data)
            } //(50049, "query process task relation error", "查询工作流任务关系错误")
            (50050, "task definition [{0}] is already online", "任务定义[{0}]已上线", ..) => {
                Error::TaskDefineStateOnline(value.error_data)
            } //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线")
            (50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖", ..) => {
                Error::TaskHasDownstream(value.error_data)
            } //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖")
            (50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖", ..) => {
                Error::TaskHasUpstream(value.error_data)
            } //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖")
            (50053, "the version that the master table is using", "主表正在使用该版本", ..) => {
                Error::MainTableUsingVersion(value.error_data)
            } //(50053, "the version that the master table is using", "主表正在使用该版本")
            (50054, "the project and the process is not match", "项目和工作流不匹配", ..) => {
                Error::ProjectProcessNotMatch(value.error_data)
            } //(50054, "the project and the process is not match", "项目和工作流不匹配")
            (50055, "delete edge error", "删除工作流任务连接线错误", ..) => {
                Error::DeleteEdgeError(value.error_data)
            } //(50055, "delete edge error", "删除工作流任务连接线错误")
            (50056, "task state does not support modification", "当前任务不支持修改", ..) => {
                Error::NotSupportUpdateTaskDefinition(value.error_data)
            } //(50056, "task state does not support modification", "当前任务不支持修改")
            (50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]", ..) => {
                Error::NotSupportCopyTaskType(value.error_data)
            } //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]")
            (60001, "hdfs not startup", "hdfs未启用", ..) => {
                Error::HdfsNotStartup(value.error_data)
            } //(60001, "hdfs not startup", "hdfs未启用")
            (60002, "storage not startup", "存储未启用", ..) => {
                Error::StorageNotStartup(value.error_data)
            } //(60002, "storage not startup", "存储未启用")
            (60003, "directory cannot be renamed", "S3无法重命名文件夹", ..) => {
                Error::S3CannotRename(value.error_data)
            } //(60003, "directory cannot be renamed", "S3无法重命名文件夹")
            // for monitor
            (70001, "query database state error", "查询数据库状态错误", ..) => {
                Error::QueryDatabaseStateError(value.error_data)
            } //(70001, "query database state error", "查询数据库状态错误")

            (70010, ..) => Error::CreateAccessTokenError(value.error_data), //(70010, "create access token error", "创建访问token错误")
            (70011, ..) => Error::GenerateTokenError(value.error_data), //(70011, "generate token error", "生成token错误")
            (70012, ..) => Error::QueryAccesstokenListPagingError(value.error_data), //(70012, "query access token list paging error", "分页查询访问token列表错误")
            (70013, ..) => Error::UpdateAccessTokenError(value.error_data), //(70013, "update access token error", "更新访问token错误")
            (70014, ..) => Error::DeleteAccessTokenError(value.error_data), //(70014, "delete access token error", "删除访问token错误")
            (70015, ..) => Error::AccessTokenNotExist(value.error_data), //(70015, "access token not exist", "访问token不存在")
            (70016, ..) => Error::QueryAccesstokenByUserError(value.error_data), //(70016, "query access token by user error", "查询访问指定用户的token错误")

            (80001, ..) => Error::CommandStateCountError(value.error_data), //(80001, "task instance state count error", "查询各状态任务实例数错误")
            (80002, ..) => Error::NegativeSizeNumberError(value.error_data), //(80002, "query size number error", "查询size错误")
            (80003, ..) => Error::StartTimeBiggerThanEndTimeError(value.error_data), //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误")
            (90001, ..) => Error::QueueCountError(value.error_data), //(90001, "queue count error", "查询队列数据错误")

            (100001, ..) => Error::KerberosStartupState(value.error_data), //(100001, "get kerberos startup state error", "获取kerberos启动状态错误")

            // audit log
            (10057, "query audit log list paging", "分页查询日志列表错误", ..) => {
                Error::QueryAuditLogListPaging(value.error_data)
            } //(10057, "query audit log list paging", "分页查询日志列表错误")

            //plugin
            (110001, ..) => Error::PluginNotAUiComponent(value.error_data), //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件")
            (110002, ..) => Error::QueryPluginsResultIsNull(value.error_data), //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功")
            (110003, ..) => Error::QueryPluginsError(value.error_data), //(110003, "query plugins error", "查询插件错误")
            (110004, ..) => Error::QueryPluginDetailResultIsNull(value.error_data), //(110004, "query plugin detail result is null", "查询插件详情结果为空")

            (110005, ..) => Error::UpdateAlertPluginInstanceError(value.error_data), //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误")
            (110006, ..) => Error::DeleteAlertPluginInstanceError(value.error_data), //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误")
            (110007, ..) => Error::GetAlertPluginInstanceError(value.error_data), //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误")
            (110008, ..) => Error::CreateAlertPluginInstanceError(value.error_data), //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误")
            (110009, ..) => Error::QueryAllAlertPluginInstanceError(value.error_data), //(110009, "query all alert plugin instance error", "查询所有告警实例失败")
            (110010, ..) => Error::PluginInstanceAlreadyExit(value.error_data), //(110010, "plugin instance already exit", "该告警插件实例已存在")
            (110011, ..) => Error::ListPagingAlertPluginInstanceError(value.error_data), //(110011, "query plugin instance page error", "分页查询告警实例失败")
            (110012, ..) => {
                Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(value.error_data)
            } //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组")
            (110013, ..) => Error::ProcessDefinitionVersionIsUsed(value.error_data), //(110013, "this process definition version is used", "此工作流定义版本被使用")

            (120001, ..) => Error::CreateEnvironmentError(value.error_data), //(120001, "create environment error", "创建环境失败")
            (120002, ..) => Error::EnvironmentNameExists(value.error_data), //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在")
            (120003, ..) => Error::EnvironmentNameIsNull(value.error_data), //(120003, "this environment name shouldn't be empty.", "环境名称不能为空")
            (120004, ..) => Error::EnvironmentConfigIsNull(value.error_data), //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空")
            (120005, ..) => Error::UpdateEnvironmentError(value.error_data), //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败")
            (120006, ..) => Error::DeleteEnvironmentError(value.error_data), //(120006, "delete environment error", "删除环境信息失败")
            (120007, ..) => Error::DeleteEnvironmentRelatedTaskExists(value.error_data), //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息")
            (1200008, ..) => Error::QueryEnvironmentByNameError(value.error_data), //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在")
            (1200009, ..) => Error::QueryEnvironmentByCodeError(value.error_data), //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在")
            (1200010, ..) => Error::QueryEnvironmentError(value.error_data), //(1200010, "login user query environment error", "分页查询环境列表错误")
            (1200011, ..) => Error::VerifyEnvironmentError(value.error_data), //(1200011, "verify environment error", "验证环境信息错误")
            (1200012, ..) => Error::GetRuleFormCreateJsonError(value.error_data), //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误")
            (1200013, ..) => Error::QueryRuleListPagingError(value.error_data), //(1200013, "query rule list paging error", "获取规则分页列表错误")
            (1200014, ..) => Error::QueryRuleListError(value.error_data), //(1200014, "query rule list error", "获取规则列表错误")
            (1200015, ..) => Error::QueryRuleInputEntryListError(value.error_data), //(1200015, "query rule list error", "获取规则列表错误")
            (1200016, ..) => Error::QueryExecuteResultListPagingError(value.error_data), //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误")
            (1200017, ..) => Error::GetDatasourceOptionsError(value.error_data), //(1200017, "get datasource options error", "获取数据源Options错误")
            (1200018, ..) => Error::GetDatasourceTablesError(value.error_data), //(1200018, "get datasource tables error", "获取数据源表列表错误")
            (1200019, ..) => Error::GetDatasourceTableColumnsError(value.error_data), //(1200019, "get datasource table columns error", "获取数据源表列名错误")
            (130001, ..) => Error::TaskGroupNameExist(value.error_data), //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用")
            (130002, ..) => Error::TaskGroupSizeError(value.error_data), //(130002, "task group size error", "任务组大小应该为大于1的整数")
            (130003, ..) => Error::TaskGroupStatusError(value.error_data), //(130003, "task group status error", "任务组已经被关闭")
            (130004, ..) => Error::TaskGroupFull(value.error_data), //(130004, "task group is full", "任务组已经满了")
            (130005, ..) => Error::TaskGroupUsedSizeError(value.error_data), //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化")
            (130006, ..) => Error::TaskGroupQueueReleaseError(value.error_data), //(130006, "failed to release task group queue", "任务组资源释放时出现了错误")
            (130007, ..) => Error::TaskGroupQueueAwakeError(value.error_data), //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误")
            (130008, ..) => Error::CreateTaskGroupError(value.error_data), //(130008, "create task group error", "创建任务组错误")
            (130009, ..) => Error::UpdateTaskGroupError(value.error_data), //(130009, "update task group list error", "更新任务组错误")
            (130010, ..) => Error::QueryTaskGroupListError(value.error_data), //(130010, "query task group list error", "查询任务组列表错误")
            (130011, ..) => Error::CloseTaskGroupError(value.error_data), //(130011, "close task group error", "关闭任务组错误")
            (130012, ..) => Error::StartTaskGroupError(value.error_data), //(130012, "start task group error", "启动任务组错误")
            (130013, ..) => Error::QueryTaskGroupQueueListError(value.error_data), //(130013, "query task group queue list error", "查询任务组队列列表错误")
            (130014, ..) => Error::TaskGroupCacheStartFailed(value.error_data), //(130014, "cache start failed", "任务组相关的缓存启动失败")
            (130015, ..) => Error::EnvironmentWorkerGroupsIsInvalid(value.error_data), //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误")
            (130016, ..) => Error::UpdateEnvironmentWorkerGroupRelationError(value.error_data), //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中")
            (130017, ..) => Error::TaskGroupQueueAlreadyStart(value.error_data), //(130017, "task group queue already start", "节点已经获取任务组资源")
            (130018, ..) => Error::TaskGroupStatusClosed(value.error_data), //(130018, "The task group has been closed.", "任务组已经被关闭")
            (130019, ..) => Error::TaskGroupStatusOpened(value.error_data), //(130019, "The task group has been opened.", "任务组已经被开启")
            (130020, ..) => Error::NotAllowToDisableOwnAccount(value.error_data), //(130020, "Not allow to disable your own account", "不能停用自己的账号")
            (130030, ..) => Error::NotAllowToDeleteDefaultAlarmGroup(value.error_data), //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组")
            (130031, ..) => Error::TimeZoneIllegal(value.error_data), //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法")

            (1300001, ..) => Error::QueryK8sNamespaceListPagingError(value.error_data), //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误")
            (1300002, ..) => Error::K8sNamespaceExist(value.error_data), //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在")
            (1300003, ..) => Error::CreateK8sNamespaceError(value.error_data), //(1300003, "create k8s namespace error", "创建k8s命名空间错误")
            (1300004, ..) => Error::UpdateK8sNamespaceError(value.error_data), //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误")
            (1300005, ..) => Error::K8sNamespaceNotExist(value.error_data), //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在")
            (1300006, ..) => Error::K8sClientOpsError(value.error_data), //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]")
            (1300007, ..) => Error::VerifyK8sNamespaceError(value.error_data), //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误")
            (1300008, ..) => Error::DeleteK8sNamespaceByIdError(value.error_data), //(1300008, "delete k8s namespace by id error", "删除命名空间错误")
            (1300009, ..) => Error::VerifyParameterNameFailed(value.error_data), //(1300009, "The file name verify failed", "文件命名校验失败")
            (1300010, ..) => Error::StoreOperateCreateError(value.error_data), //(1300010, "create the resource failed", "存储操作失败")
            (1300011, ..) => Error::GrantK8sNamespaceError(value.error_data), //(1300011, "grant namespace error", "授权资源错误")
            (1300012, ..) => Error::QueryUnauthorizedNamespaceError(value.error_data), //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误")
            (1300013, ..) => Error::QueryAuthorizedNamespaceError(value.error_data), //(1300013, "query authorized namespace error", "查询授权命名空间错误")
            (1300014, ..) => Error::QueryCanUseK8sClusterError(value.error_data), //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误")
            (1300015, ..) => Error::ResourceFullNameTooLongError(value.error_data), //(1300015, "resource's fullname is too long error", "资源文件名过长")
            (1300016, ..) => Error::TenantFullNameTooLongError(value.error_data), //(1300016, "tenant's fullname is too long error", "租户名过长");
            (..) => Error::InternalServerErrorArgs(value.error_data),
        }
    }
}

impl From<Error> for AuroraErrorInfo {
    fn from(status: Error) -> Self {
        match status {
            Error::SUCCESS(_) => AuroraErrorInfo::new(0, "success".to_string(), "成功".to_string()),
            Error::InternalServerErrorArgs(data) => AuroraErrorInfo::new(
                10000,
                "internal server error please check the log".to_string(),
                "内部服务错误，请查看日志".to_string(),
            ).new_with_data(data),
            Error::RequestParamsNotValidError(data) => AuroraErrorInfo::new(
                10001,
                "request parameter {0} is not valid".to_string(),
                "请求参数[{0}]无效".to_string(),
            ).new_with_data(data),
            Error::TaskTimeoutParamsError (data)=> AuroraErrorInfo::new(
                10002,
                "task timeout parameter is not valid".to_string(),
                "任务超时参数无效".to_string(),
            ).new_with_data(data),
            Error::UserNameExist (data)=> AuroraErrorInfo::new(
                10003,
                "user name already exists".to_string(),
                "用户名已存在".to_string(),
            ).new_with_data(data),
            Error::UserNameNull (data)=> AuroraErrorInfo::new(
                10004,
                "user name is null".to_string(),
                "用户名不能为空".to_string(),
            ).new_with_data(data),
            Error::HdfsOperationError (data)=> AuroraErrorInfo::new(
                10006,
                "hdfs operation error".to_string(),
                "hdfs操作错误".to_string(),
            ).new_with_data(data),
            Error::TaskInstanceNotFound (data)=> AuroraErrorInfo::new(
                10008,
                "task instance not found".to_string(),
                "任务实例不存在".to_string(),
            ).new_with_data(data),
            Error::OsTenantCodeExist (data)=> AuroraErrorInfo::new(
                10009,
                "os tenant code {0} already exists".to_string(),
                "操作系统租户[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::UserNotExist (data)=> AuroraErrorInfo::new(
                10010,
                "user {0} not exists".to_string(),
                "用户[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::AlertGroupNotExist (data)=> AuroraErrorInfo::new(
                10011,
                "alarm group not found".to_string(),
                "告警组不存在".to_string(),
            ).new_with_data(data),
            Error::AlertGroupExist (data)=> AuroraErrorInfo::new(
                10012,
                "alarm group already exists".to_string(),
                "告警组名称已存在".to_string(),
            ).new_with_data(data),
            Error::UserNamePasswdError (data)=> AuroraErrorInfo::new(
                10013,
                "user name or password error".to_string(),
                "用户名或密码错误".to_string(),
            ).new_with_data(data),
            Error::LoginSessionFailed (data)=> AuroraErrorInfo::new(
                10014,
                "create session failed!".to_string(),
                "创建session失败".to_string(),
            ).new_with_data(data),
            Error::DatasourceExist (data)=> AuroraErrorInfo::new(
                10015,
                "data source name already exists".to_string(),
                "数据源名称已存在".to_string(),
            ).new_with_data(data),
            Error::DatasourceConnectFailed (data)=> AuroraErrorInfo::new(
                10016,
                "data source connection failed".to_string(),
                "建立数据源连接失败".to_string(),
            ).new_with_data(data),
            Error::TenantNotExist (data)=> AuroraErrorInfo::new(
                10017,
                "tenant not exists".to_string(),
                "租户不存在".to_string(),
            ).new_with_data(data),
            Error::ProjectNotFound (data)=> AuroraErrorInfo::new(
                10018,
                "project {0} not found ".to_string(),
                "项目[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::ProjectAlreadyExists (data)=> AuroraErrorInfo::new(
                10019,
                "project {0} already exists".to_string(),
                "项目名称[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::TaskInstanceNotExists (data)=> AuroraErrorInfo::new(
                10020,
                "task instance {0} does not exist".to_string(),
                "任务实例[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::TaskInstanceNotSubWorkflowInstance (data)=> AuroraErrorInfo::new(
                10021,
                "task instance {0} is not sub process instance".to_string(),
                "任务实例[{0}]不是子流程实例".to_string(),
            ).new_with_data(data),
            Error::ScheduleCronNotExists (data)=> AuroraErrorInfo::new(
                10022,
                "scheduler crontab {0} does not exist".to_string(),
                "调度配置定时表达式[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::ScheduleCronOnlineForbidUpdate (data)=> AuroraErrorInfo::new(
                10023,
                "online status does not allow update operations".to_string(),
                "调度配置上线状态不允许修改".to_string(),
            ).new_with_data(data),
            Error::ScheduleCronCheckFailed (data)=> AuroraErrorInfo::new(
                10024,
                "scheduler crontab expression validation failure: {0}".to_string(),
                "调度配置定时表达式验证失败: {0}".to_string(),
            ).new_with_data(data),
            Error::MasterNotExists (data)=> AuroraErrorInfo::new(
                10025,
                "master does not exist".to_string(),
                "无可用master节点".to_string(),
            ).new_with_data(data),
            Error::ScheduleStatusUnknown (data)=> AuroraErrorInfo::new(
                10026,
                "unknown status: {0}".to_string(),
                "未知状态: {0}".to_string(),
            ).new_with_data(data),
            Error::CreateAlertGroupError (data)=> AuroraErrorInfo::new(
                10027,
                "create alert group error".to_string(),
                "创建告警组错误".to_string(),
            ).new_with_data(data),
            Error::QueryAllAlertgroupError (data)=> AuroraErrorInfo::new(
                10028,
                "query all alertgroup error".to_string(),
                "查询告警组错误".to_string(),
            ).new_with_data(data),
            Error::ListPagingAlertGroupError (data)=> AuroraErrorInfo::new(
                10029,
                "list paging alert group error".to_string(),
                "分页查询告警组错误".to_string(),
            ).new_with_data(data),
            Error::UpdateAlertGroupError (data)=> AuroraErrorInfo::new(
                10030,
                "update alert group error".to_string(),
                "更新告警组错误".to_string(),
            ).new_with_data(data),
            Error::DeleteAlertGroupError (data)=> AuroraErrorInfo::new(
                10031,
                "delete alert group error".to_string(),
                "删除告警组错误".to_string(),
            ).new_with_data(data),
            Error::AlertGroupGrantUserError (data)=> AuroraErrorInfo::new(
                10032,
                "alert group grant user error".to_string(),
                "告警组授权用户错误".to_string(),
            ).new_with_data(data),
            Error::CreateDatasourceError (data)=> AuroraErrorInfo::new(
                10033,
                "create datasource error".to_string(),
                "创建数据源错误".to_string(),
            ).new_with_data(data),
            Error::UpdateDatasourceError (data)=> AuroraErrorInfo::new(
                10034,
                "update datasource error".to_string(),
                "更新数据源错误".to_string(),
            ).new_with_data(data),
            Error::QueryDatasourceError (data)=> AuroraErrorInfo::new(
                10035,
                "query datasource error".to_string(),
                "查询数据源错误".to_string(),
            ).new_with_data(data),
            Error::ConnectDatasourceFailure (data)=> AuroraErrorInfo::new(
                10036,
                "connect datasource failure".to_string(),
                "建立数据源连接失败".to_string(),
            ).new_with_data(data),
            Error::ConnectionTestFailure (data)=> AuroraErrorInfo::new(
                10037,
                "connection test failure".to_string(),
                "测试数据源连接失败".to_string(),
            ).new_with_data(data),
            Error::DeleteDataSourceFailure (data)=> AuroraErrorInfo::new(
                10038,
                "delete data source failure".to_string(),
                "删除数据源失败".to_string(),
            ).new_with_data(data),
            Error::VerifyDatasourceNameFailure (data)=> AuroraErrorInfo::new(
                10039,
                "verify datasource name failure".to_string(),
                "验证数据源名称失败".to_string(),
            ).new_with_data(data),
            Error::UnauthorizedDatasource (data)=> AuroraErrorInfo::new(
                10040,
                "unauthorized datasource".to_string(),
                "未经授权的数据源".to_string(),
            ).new_with_data(data),
            Error::AuthorizedDataSource (data)=> AuroraErrorInfo::new(
                10041,
                "authorized data source".to_string(),
                "授权数据源失败".to_string(),
            ).new_with_data(data),
            Error::LoginSuccess (data)=> {
                AuroraErrorInfo::new(10042, "login success".to_string(), "登录成功".to_string()).new_with_data(data)
            }
            Error::UserLoginFailure (data)=> AuroraErrorInfo::new(
                10043,
                "user login failure".to_string(),
                "用户登录失败".to_string(),
            ).new_with_data(data),
            Error::ListWorkersError (data)=> AuroraErrorInfo::new(
                10044,
                "list workers error".to_string(),
                "查询worker列表错误".to_string(),
            ).new_with_data(data),
            Error::ListMastersError (data)=> AuroraErrorInfo::new(
                10045,
                "list masters error".to_string(),
                "查询master列表错误".to_string(),
            ).new_with_data(data),
            Error::UpdateProjectError (data)=> AuroraErrorInfo::new(
                10046,
                "update project error".to_string(),
                "更新项目信息错误".to_string(),
            ).new_with_data(data),
            Error::QueryProjectDetailsByCodeError (data)=> AuroraErrorInfo::new(
                10047,
                "query project details by code error".to_string(),
                "查询项目详细信息错误".to_string(),
            ).new_with_data(data),
            Error::CreateProjectError (data)=> AuroraErrorInfo::new(
                10048,
                "create project error".to_string(),
                "创建项目错误".to_string(),
            ).new_with_data(data),
            Error::LoginUserQueryProjectListPagingError (data)=> AuroraErrorInfo::new(
                10049,
                "login user query project list paging error".to_string(),
                "分页查询项目列表错误".to_string(),
            ).new_with_data(data),
            Error::DeleteProjectError (data)=> AuroraErrorInfo::new(
                10050,
                "delete project error".to_string(),
                "删除项目错误".to_string(),
            ).new_with_data(data),
            Error::QueryUnauthorizedProjectError (data)=> AuroraErrorInfo::new(
                10051,
                "query unauthorized project error".to_string(),
                "查询未授权项目错误".to_string(),
            ).new_with_data(data),
            Error::QueryAuthorizedProject (data)=> AuroraErrorInfo::new(
                10052,
                "query authorized project".to_string(),
                "查询授权项目错误".to_string(),
            ).new_with_data(data),
            Error::QueryQueueListError (data)=> AuroraErrorInfo::new(
                10053,
                "query queue list error".to_string(),
                "查询队列列表错误".to_string(),
            ).new_with_data(data),
            Error::CreateResourceError (data)=> AuroraErrorInfo::new(
                10054,
                "create resource error".to_string(),
                "创建资源错误".to_string(),
            ).new_with_data(data),
            Error::UpdateResourceError (data)=> AuroraErrorInfo::new(
                10055,
                "update resource error".to_string(),
                "更新资源错误".to_string(),
            ).new_with_data(data),
            Error::QueryResourcesListError (data)=> AuroraErrorInfo::new(
                10056,
                "query resources list error".to_string(),
                "查询资源列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryResourcesListPaging (data)=> AuroraErrorInfo::new(
                10057,
                "query resources list paging".to_string(),
                "分页查询资源列表错误".to_string(),
            ).new_with_data(data),
            Error::DeleteResourceError (data)=> AuroraErrorInfo::new(
                10058,
                "delete resource error".to_string(),
                "删除资源错误".to_string(),
            ).new_with_data(data),
            Error::VerifyResourceByNameAndTypeError (data)=> AuroraErrorInfo::new(
                10059,
                "verify resource by name and type error".to_string(),
                "资源名称或类型验证错误".to_string(),
            ).new_with_data(data),
            Error::ViewResourceFileOnLineError (data)=> AuroraErrorInfo::new(
                10060,
                "view resource file online error".to_string(),
                "查看资源文件错误".to_string(),
            ).new_with_data(data),
            Error::CreateResourceFileOnLineError (data)=> AuroraErrorInfo::new(
                10061,
                "create resource file online error".to_string(),
                "创建资源文件错误".to_string(),
            ).new_with_data(data),
            Error::ResourceFileIsEmpty (data)=> AuroraErrorInfo::new(
                10062,
                "resource file is empty".to_string(),
                "资源文件内容不能为空".to_string(),
            ).new_with_data(data),
            Error::EditResourceFileOnLineError (data)=> AuroraErrorInfo::new(
                10063,
                "edit resource file online error".to_string(),
                "更新资源文件错误".to_string(),
            ).new_with_data(data),
            Error::DownloadResourceFileError (data)=> AuroraErrorInfo::new(
                10064,
                "download resource file error".to_string(),
                "下载资源文件错误".to_string(),
            ).new_with_data(data),
            Error::CreateUdfFunctionError (data)=> AuroraErrorInfo::new(
                10065,
                "create udf function error".to_string(),
                "创建UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::ViewUdfFunctionError (data)=> AuroraErrorInfo::new(
                10066,
                "view udf function error".to_string(),
                "查询UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::UpdateUdfFunctionError (data)=> AuroraErrorInfo::new(
                10067,
                "update udf function error".to_string(),
                "更新UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::QueryUdfFunctionListPagingError (data)=> AuroraErrorInfo::new(
                10068,
                "query udf function list paging error".to_string(),
                "分页查询UDF函数列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryDatasourceByTypeError (data)=> AuroraErrorInfo::new(
                10069,
                "query datasource by type error".to_string(),
                "查询数据源信息错误".to_string(),
            ).new_with_data(data),
            Error::VerifyUdfFunctionNameError (data)=> AuroraErrorInfo::new(
                10070,
                "verify udf function name error".to_string(),
                "UDF函数名称验证错误".to_string(),
            ).new_with_data(data),
            Error::DeleteUdfFunctionError (data)=> AuroraErrorInfo::new(
                10071,
                "delete udf function error".to_string(),
                "删除UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::AuthorizedFileResourceError (data)=> AuroraErrorInfo::new(
                10072,
                "authorized file resource error".to_string(),
                "授权资源文件错误".to_string(),
            ).new_with_data(data),
            Error::AuthorizeResourceTree (data)=> AuroraErrorInfo::new(
                10073,
                "authorize resource tree display error".to_string(),
                "授权资源目录树错误".to_string(),
            ).new_with_data(data),
            Error::UnauthorizedUdfFunctionError (data)=> AuroraErrorInfo::new(
                10074,
                "unauthorized udf function error".to_string(),
                "查询未授权UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::AuthorizedUdfFunctionError (data)=> AuroraErrorInfo::new(
                10075,
                "authorized udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::CreateScheduleError (data)=> AuroraErrorInfo::new(
                10076,
                "create schedule error".to_string(),
                "创建调度配置错误".to_string(),
            ).new_with_data(data),
            Error::UpdateScheduleError (data)=> AuroraErrorInfo::new(
                10077,
                "update schedule error".to_string(),
                "更新调度配置错误".to_string(),
            ).new_with_data(data),
            Error::PublishScheduleOnlineError (data)=> AuroraErrorInfo::new(
                10078,
                "publish schedule online error".to_string(),
                "上线调度配置错误".to_string(),
            ).new_with_data(data),
            Error::OfflineScheduleError (data)=> AuroraErrorInfo::new(
                10079,
                "offline schedule error".to_string(),
                "下线调度配置错误".to_string(),
            ).new_with_data(data),
            Error::QueryScheduleListPagingError (data)=> AuroraErrorInfo::new(
                10080,
                "query schedule list paging error".to_string(),
                "分页查询调度配置列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryScheduleListError (data)=> AuroraErrorInfo::new(
                10081,
                "query schedule list error".to_string(),
                "查询调度配置列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskListPagingError (data)=> AuroraErrorInfo::new(
                10082,
                "query task list paging error".to_string(),
                "分页查询任务列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskRecordListPagingError (data)=> AuroraErrorInfo::new(
                10083,
                "query task record list paging error".to_string(),
                "分页查询任务记录错误".to_string(),
            ).new_with_data(data),
            Error::CreateTenantError (data)=> AuroraErrorInfo::new(
                10084,
                "create tenant error".to_string(),
                "创建租户错误".to_string(),
            ).new_with_data(data),
            Error::QueryTenantListPagingError (data)=> AuroraErrorInfo::new(
                10085,
                "query tenant list paging error".to_string(),
                "分页查询租户列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryTenantListError (data)=> AuroraErrorInfo::new(
                10086,
                "query tenant list error".to_string(),
                "查询租户列表错误".to_string(),
            ).new_with_data(data),
            Error::UpdateTenantError (data)=> AuroraErrorInfo::new(
                10087,
                "update tenant error".to_string(),
                "更新租户错误".to_string(),
            ).new_with_data(data),
            Error::DeleteTenantByIdError (data)=> AuroraErrorInfo::new(
                10088,
                "delete tenant by id error".to_string(),
                "删除租户错误".to_string(),
            ).new_with_data(data),
            Error::VerifyOsTenantCodeError (data)=> AuroraErrorInfo::new(
                10089,
                "verify os tenant code error".to_string(),
                "操作系统租户验证错误".to_string(),
            ).new_with_data(data),
            Error::CreateUserError (data)=> AuroraErrorInfo::new(
                10090,
                "create user error".to_string(),
                "创建用户错误".to_string(),
            ).new_with_data(data),
            Error::QueryUserListPagingError (data)=> AuroraErrorInfo::new(
                10091,
                "query user list paging error".to_string(),
                "分页查询用户列表错误".to_string(),
            ).new_with_data(data),
            Error::UpdateUserError (data)=> AuroraErrorInfo::new(
                10092,
                "update user error".to_string(),
                "更新用户错误".to_string(),
            ).new_with_data(data),
            Error::DeleteUserByIdError (data)=> AuroraErrorInfo::new(
                10093,
                "delete user by id error".to_string(),
                "删除用户错误".to_string(),
            ).new_with_data(data),
            Error::GrantProjectError (data)=> AuroraErrorInfo::new(
                10094,
                "grant project error".to_string(),
                "授权项目错误".to_string(),
            ).new_with_data(data),
            Error::GrantResourceError (data)=> AuroraErrorInfo::new(
                10095,
                "grant resource error".to_string(),
                "授权资源错误".to_string(),
            ).new_with_data(data),
            Error::GrantUdfFunctionError (data)=> AuroraErrorInfo::new(
                10096,
                "grant udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            ).new_with_data(data),
            Error::GrantDatasourceError (data)=> AuroraErrorInfo::new(
                10097,
                "grant datasource error".to_string(),
                "授权数据源错误".to_string(),
            ).new_with_data(data),
            Error::GetUserInfoError (data)=> AuroraErrorInfo::new(
                10098,
                "get user info error".to_string(),
                "获取用户信息错误".to_string(),
            ).new_with_data(data),
            Error::UserListError (data)=> AuroraErrorInfo::new(
                10099,
                "user list error".to_string(),
                "查询用户列表错误".to_string(),
            ).new_with_data(data),
            Error::VerifyUsernameError (data)=> AuroraErrorInfo::new(
                10100,
                "verify username error".to_string(),
                "用户名验证错误".to_string(),
            ).new_with_data(data),
            Error::UnauthorizedUserError (data)=> AuroraErrorInfo::new(
                10101,
                "unauthorized user error".to_string(),
                "查询未授权用户错误".to_string(),
            ).new_with_data(data),
            Error::AuthorizedUserError (data)=> AuroraErrorInfo::new(
                10102,
                "authorized user error".to_string(),
                "查询授权用户错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskInstanceLogError (data)=> AuroraErrorInfo::new(
                10103,
                "view task instance log error".to_string(),
                "查询任务实例日志错误".to_string(),
            ).new_with_data(data),
            Error::DownloadTaskInstanceLogFileError (data)=> AuroraErrorInfo::new(
                10104,
                "download task instance log file error".to_string(),
                "下载任务日志文件错误".to_string(),
            ).new_with_data(data),
            Error::CreateProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10105,
                "create process definition error".to_string(),
                "创建工作流错误".to_string(),
            ).new_with_data(data),
            Error::VerifyProcessDefinitionNameUniqueError (data)=> AuroraErrorInfo::new(
                10106,
                "verify process definition name unique error".to_string(),
                "工作流定义名称验证错误".to_string(),
            ).new_with_data(data),
            Error::UpdateProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10107,
                "update process definition error".to_string(),
                "更新工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::ReleaseProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10108,
                "release process definition error".to_string(),
                "上线工作流错误".to_string(),
            ).new_with_data(data),
            Error::QueryDetailOfProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10109,
                "query detail of process definition error".to_string(),
                "查询工作流详细信息错误".to_string(),
            ).new_with_data(data),
            Error::QueryProcessDefinitionList (data)=> AuroraErrorInfo::new(
                10110,
                "query process definition list".to_string(),
                "查询工作流列表错误".to_string(),
            ).new_with_data(data),
            Error::EncapsulationTreeviewStructureError (data)=> AuroraErrorInfo::new(
                10111,
                "encapsulation treeview structure error".to_string(),
                "查询工作流树形图数据错误".to_string(),
            ).new_with_data(data),
            Error::GetTasksListByProcessDefinitionIdError (data)=> AuroraErrorInfo::new(
                10112,
                "get tasks list by process definition id error".to_string(),
                "查询工作流定义节点信息错误".to_string(),
            ).new_with_data(data),
            Error::QueryProcessInstanceListPagingError (data)=> AuroraErrorInfo::new(
                10113,
                "query process instance list paging error".to_string(),
                "分页查询工作流实例列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskListByProcessInstanceIdError (data)=> AuroraErrorInfo::new(
                10114,
                "query task list by process instance id error".to_string(),
                "查询任务实例列表错误".to_string(),
            ).new_with_data(data),
            Error::UpdateProcessInstanceError (data)=> AuroraErrorInfo::new(
                10115,
                "update process instance error".to_string(),
                "更新工作流实例错误".to_string(),
            ).new_with_data(data) ,
            Error::QueryProcessInstanceByIdError (data)=> AuroraErrorInfo::new(
                10116,
                "query process instance by id error".to_string(),
                "查询工作流实例错误".to_string(),
            ).new_with_data(data) ,
            Error::DeleteProcessInstanceByIdError (data)=> AuroraErrorInfo::new(
                10117,
                "delete process instance by id error".to_string(),
                "删除工作流实例错误".to_string(),
            ).new_with_data(data) ,
            Error::QuerySubProcessInstanceDetailInfoByTaskIdError (data)=> AuroraErrorInfo::new(
                10118,
                "query sub process instance detail info by task id error".to_string(),
                "查询子流程任务实例错误".to_string(),
            ).new_with_data(data) ,
            Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError (data)=>AuroraErrorInfo::new(
                    10119,
                    "query parent process instance detail info by sub process instance id error"
                        .to_string(),
                    "查询子流程该工作流实例错误".to_string(),
                ).new_with_data(data),
            Error::QueryProcessInstanceAllVariablesError (data)=> AuroraErrorInfo::new(
                10120,
                "query process instance all variables error".to_string(),
                "查询工作流自定义变量信息错误".to_string(),
            ).new_with_data(data) ,
            Error::EncapsulationProcessInstanceGanttStructureError (data)=> AuroraErrorInfo::new(
                10121,
                "encapsulation process instance gantt structure error".to_string(),
                "查询工作流实例甘特图数据错误".to_string(),
            ).new_with_data(data) ,
            Error::QueryProcessDefinitionListPagingError (data)=> AuroraErrorInfo::new(
                10122,
                "query process definition list paging error".to_string(),
                "分页查询工作流定义列表错误".to_string(),
            ).new_with_data(data) ,
            Error::SignOutError (data)=> {
                AuroraErrorInfo::new(10123, "sign out error".to_string(), "退出错误".to_string()) .new_with_data(data) 
            }
            Error::OsTenantCodeHasAlreadyExists (data)=> AuroraErrorInfo::new(
                10124,
                "os tenant code has already exists".to_string(),
                "操作系统租户已存在".to_string(),
            ).new_with_data(data),
            Error::IpIsEmpty (data)=> AuroraErrorInfo::new(
                10125,
                "ip is empty".to_string(),
                "IP地址不能为空".to_string(),
            ).new_with_data(data),
            Error::ScheduleCronReleaseNeedNotChange (data)=> AuroraErrorInfo::new(
                10126,
                "schedule release is already {0}".to_string(),
                "调度配置上线错误[{0}]".to_string(),
            ).new_with_data(data),
            Error::CreateQueueError (data)=> AuroraErrorInfo::new(
                10127,
                "create queue error".to_string(),
                "创建队列错误".to_string(),
            ) .new_with_data(data),
            Error::QueueNotExist (data)=> AuroraErrorInfo::new(
                10128,
                "queue {0} not exists".to_string(),
                "队列ID[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::QueueValueExist (data)=> AuroraErrorInfo::new(
                10129,
                "queue value {0} already exists".to_string(),
                "队列值[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::QueueNameExist (data)=> AuroraErrorInfo {
                code: 10130,
                en_msg: "queue name {0} already exists".to_string(),
                cn_msg: "队列名称[{0}]已存在".to_string(),
                error_data:data,
            },
            Error::UpdateQueueError (data)=> AuroraErrorInfo::new(
                10131,
                "update queue error".to_string(),
                "更新队列信息错误".to_string(),
            ).new_with_data(data),
            Error::NeedNotUpdateQueue (data)=> AuroraErrorInfo::new(
                10132,
                "need not update queue".to_string(),
                "无需更新队列信息".to_string(),
            ).new_with_data(data),
            Error::VerifyQueueError (data)=> AuroraErrorInfo::new(
                10133,
                "verify queue error".to_string(),
                "验证队列信息错误".to_string(),
            ).new_with_data(data),
            Error::NameNull (data)=> AuroraErrorInfo::new(
                10134,
                "name must be not null".to_string(),
                "名称不能为空".to_string(),
            ).new_with_data(data),
            Error::NameExist (data)=> AuroraErrorInfo::new(
                10135,
                "name {0} already exists".to_string(),
                "名称[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::SaveError (data)=> {
                AuroraErrorInfo::new(10136, "save error".to_string(), "保存错误".to_string()).new_with_data(data)
            }
            Error::DeleteProjectErrorDefinesNotNull (data)=> AuroraErrorInfo::new(
                10117,
                "please delete the process definitions in project first!".to_string(),
                "请先删除全部工作流定义".to_string(),
            ).new_with_data(data),
            Error::BatchDeleteProcessInstanceByIdsError (data)=> AuroraErrorInfo::new(
                10138,
                "batch delete process instance by ids {0} error".to_string(),
                "批量删除工作流实例错误: {0}".to_string(),
            ).new_with_data(data),
            Error::PreviewScheduleError (data)=> AuroraErrorInfo::new(
                10139,
                "preview schedule error".to_string(),
                "预览调度配置错误".to_string(),
            ).new_with_data(data),
            Error::ParseToCronExpressionError (data)=> AuroraErrorInfo::new(
                10140,
                "parse cron to cron expression error".to_string(),
                "解析调度表达式错误".to_string(),
            ).new_with_data(data),
            Error::ScheduleStartTimeEndTimeSame (data)=> AuroraErrorInfo::new(
                10141,
                "The start time must not be the same as the end".to_string(),
                "开始时间不能和结束时间一样".to_string(),
            ).new_with_data(data),
            Error::DeleteTenantByIdFail (data)=> AuroraErrorInfo::new(
                10142,
                "delete tenant by id fail:for there are {0} process instances in executing using \
                 it"
                .to_string(),
                "删除租户失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ).new_with_data(data),
            Error::DeleteTenantByIdFailDefines (data)=> AuroraErrorInfo::new(
                10143,
                "delete tenant by id fail:for there are {0} process definitions using it"
                    .to_string(),
                "删除租户失败，有[{0}]个工作流定义正在使用".to_string(),
            ).new_with_data(data),
            Error::DeleteTenantByIdFailUsers (data)=> AuroraErrorInfo::new(
                10144,
                "delete tenant by id fail: for there are {0} users using it".to_string(),
                "删除租户失败，有[{0}]个用户正在使用".to_string(),
            ).new_with_data(data),
            Error::DeleteWorkerGroupByIdFail (data)=> AuroraErrorInfo::new(
                10145,
                "delete worker group by id failfor there are {0} process instances in executing \
                 using it"
                    .to_string(),
                "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ).new_with_data(data),
            Error::QueryWorkerGroupFail (data)=> AuroraErrorInfo::new(
                10146,
                "query worker group fail ".to_string(),
                "查询worker分组失败".to_string(),
            ).new_with_data(data),
            Error::DeleteWorkerGroupFail (data)=> AuroraErrorInfo::new(
                10147,
                "delete worker group fail ".to_string(),
                "删除worker分组失败".to_string(),
            ).new_with_data(data),
            Error::UserDisabled (data)=> AuroraErrorInfo::new(
                10148,
                "The current user is disabled".to_string(),
                "当前用户已停用".to_string(),
            ).new_with_data(data),
            Error::CopyProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10149,
                "copy process definition from {0} to {1} error : {2}".to_string(),
                "从{0}复制工作流到{1}错误 : {2}".to_string(),
            ).new_with_data(data),
            Error::MoveProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10150,
                "move process definition from {0} to {1} error : {2}".to_string(),
                "从{0}移动工作流到{1}错误 : {2}".to_string(),
            ).new_with_data(data),
            Error::SwitchProcessDefinitionVersionError (data)=> AuroraErrorInfo::new(
                10151,
                "Switch process definition version error".to_string(),
                "切换工作流版本出错".to_string(),
            ).new_with_data(data),
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError (data)=> {
                AuroraErrorInfo::new(
                    10152,
                    "Switch process definition version error: not exists process definition: \
                     [process definition id {0}]"
                        .to_string(),
                    "切换工作流版本出错：工作流不存在，[工作流id {0}]".to_string(),
                ).new_with_data(data)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError (data)=> {
                AuroraErrorInfo::new(
                    10153,
                    "Switch process defi:nition version error: not exists process definition \
                     version: [process definition id {0}] [version number {1}]"
                        .to_string(),
                    "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"
                        .to_string(),
                ).new_with_data(data)
            }
            Error::QueryProcessDefinitionVersionsError (data)=> AuroraErrorInfo::new(
                10154,
                "query process definition versions error".to_string(),
                "查询工作流历史版本信息出错".to_string(),
            ).new_with_data(data),
            Error::DeleteProcessDefinitionVersionError (data)=> AuroraErrorInfo::new(
                10156,
                "delete process definition version error".to_string(),
                "删除工作流历史版本出错".to_string(),
            ).new_with_data(data),
            Error::QueryUserCreatedProjectError (data)=> AuroraErrorInfo::new(
                10157,
                "query user created project error error".to_string(),
                "查询用户创建的项目错误".to_string(),
            ).new_with_data(data),
            Error::ProcessDefinitionCodesIsEmpty (data)=> AuroraErrorInfo::new(
                10158,
                "process definition codes is empty".to_string(),
                "工作流CODES不能为空".to_string(),
            ).new_with_data(data),
            Error::BatchCopyProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10159,
                "batch copy process definition error".to_string(),
                "复制工作流错误".to_string(),
            ).new_with_data(data),
            Error::BatchMoveProcessDefinitionError (data)=> AuroraErrorInfo::new(
                10160,
                "batch move process definition error".to_string(),
                "移动工作流错误".to_string(),
            ).new_with_data(data),
            Error::QueryWorkflowLineageError (data)=> AuroraErrorInfo::new(
                10161,
                "query workflow lineage error".to_string(),
                "查询血缘失败".to_string(),
            ).new_with_data(data),
            Error::QueryAuthorizedAndUserCreatedProjectError (data)=> AuroraErrorInfo::new(
                10162,
                "query authorized and user created project error error".to_string(),
                "查询授权的和用户创建的项目错误".to_string(),
            ).new_with_data(data),
            Error::DeleteProcessDefinitionByCodeFail (data)=> AuroraErrorInfo::new(
                10163,
                "delete process definition by code fail.to_string(), for there are {0} process \
                 instances in executing using it"
                    .to_string(),
                "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ).new_with_data(data),
            Error::CheckOsTenantCodeError (data)=> AuroraErrorInfo::new(
                10164,
                "Tenant code invalid.to_string(), should follow linux's users naming conventions"
                    .to_string(),
                "非法的租户名，需要遵守 Linux 用户命名规范".to_string(),
            ).new_with_data(data),
            Error::ForceTaskSuccessError (data)=> AuroraErrorInfo::new(
                10165,
                "force task success error".to_string(),
                "强制成功任务实例错误".to_string(),
            ).new_with_data(data),
            Error::TaskInstanceStateOperationError (data)=> AuroraErrorInfo::new(
                10166,
                "the status of task instance {0} is {1}.to_string(),Cannot perform force success \
                 operation"
                    .to_string(),
                "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作".to_string(),
            ).new_with_data(data),
            Error::DatasourceTypeNotExist (data)=> AuroraErrorInfo::new(
                10167,
                "data source type not exist".to_string(),
                "数据源类型不存在".to_string(),
            ).new_with_data(data),
            Error::ProcessDefinitionNameExist (data)=> AuroraErrorInfo::new(
                10168,
                "process definition name {0} already exists".to_string(),
                "工作流定义名称[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::DatasourceDbTypeIllegal (data)=> AuroraErrorInfo::new(
                10169,
                "datasource type illegal".to_string(),
                "数据源类型参数不合法".to_string(),
            ).new_with_data(data),
            Error::DatasourcePortIllegal (data)=> AuroraErrorInfo::new(
                10170,
                "datasource port illegal".to_string(),
                "数据源端口参数不合法".to_string(),
            ).new_with_data(data),
            Error::DatasourceOtherParamsIllegal (data)=> AuroraErrorInfo::new(
                10171,
                "datasource other params illegal".to_string(),
                "数据源其他参数不合法".to_string(),
            ).new_with_data(data),
            Error::DatasourceNameIllegal (data)=> AuroraErrorInfo::new(
                10172,
                "datasource name illegal".to_string(),
                "数据源名称不合法".to_string(),
            ).new_with_data(data),
            Error::DatasourceHostIllegal (data)=> AuroraErrorInfo::new(
                10173,
                "datasource host illegal".to_string(),
                "数据源HOST不合法".to_string(),
            ).new_with_data(data),
            Error::DeleteWorkerGroupNotExist (data)=> AuroraErrorInfo::new(
                10174,
                "delete worker group not exist ".to_string(),
                "删除worker分组不存在".to_string(),
            ).new_with_data(data),
            Error::CreateWorkerGroupForbiddenInDocker (data)=> AuroraErrorInfo::new(
                10175,
                "create worker group forbidden in docker ".to_string(),
                "创建worker分组在docker中禁止".to_string(),
            ).new_with_data(data),
            Error::DeleteWorkerGroupForbiddenInDocker (data)=> AuroraErrorInfo::new(
                10176,
                "delete worker group forbidden in docker ".to_string(),
                "删除worker分组在docker中禁止".to_string(),
            ).new_with_data(data),
            Error::WorkerAddressInvalid (data)=> AuroraErrorInfo::new(
                10177,
                "worker address {0} invalid".to_string(),
                "worker地址[{0}]无效".to_string(),
            ).new_with_data(data),
            Error::QueryWorkerAddressListFail (data)=> AuroraErrorInfo::new(
                10178,
                "query worker address list fail ".to_string(),
                "查询worker地址列表失败".to_string(),
            ).new_with_data(data),
            Error::TransformProjectOwnership (data)=> AuroraErrorInfo::new(
                10179,
                "Please transform project ownership [{0}]".to_string(),
                "请先转移项目所有权[{0}]".to_string(),
            ).new_with_data(data),
            Error::QueryAlertGroupError (data)=> AuroraErrorInfo::new(
                10180,
                "query alert group error".to_string(),
                "查询告警组错误".to_string(),
            ).new_with_data(data),
            Error::CurrentLoginUserTenantNotExist (data)=> AuroraErrorInfo::new(
                10181,
                "the tenant of the currently login user is not specified".to_string(),
                "未指定当前登录用户的租户".to_string(),
            ).new_with_data(data),
            Error::RevokeProjectError (data)=> AuroraErrorInfo::new(
                10182,
                "revoke project error".to_string(),
                "撤销项目授权错误".to_string(),
            ).new_with_data(data),
            Error::QueryAuthorizedUser (data)=> AuroraErrorInfo::new(
                10183,
                "query authorized user error".to_string(),
                "查询拥有项目权限的用户错误".to_string(),
            ).new_with_data(data),
            Error::ProjectNotExist (data)=> AuroraErrorInfo::new(
                10190,
                "This project was not found. Please refresh page.".to_string(),
                "该项目不存在".to_string(),
            ).new_with_data(data),

            Error::TaskInstanceHostIsNull (data)=> AuroraErrorInfo::new(
                10191,
                "task instance host is null ".to_string(),
                "任务实例host为空".to_string(),
            ).new_with_data(data),
            Error::QueryExecutingWorkflowError (data)=> AuroraErrorInfo::new(
                10192,
                "query executing workflow error".to_string(),
                "查询运行的工作流实例错误".to_string(),
            ).new_with_data(data),
            Error::UdfFunctionNotExist (data)=> AuroraErrorInfo::new(
                20001,
                "UDF function not found".to_string(),
                "UDF函数不存在".to_string(),
            ).new_with_data(data),
            Error::UdfFunctionExists (data)=> AuroraErrorInfo::new(
                20002,
                "UDF function already exists".to_string(),
                "UDF函数已存在".to_string(),
            ).new_with_data(data),
            Error::ResourceNotExist (data)=> AuroraErrorInfo::new(
                20004,
                "resource not exist".to_string(),
                "资源不存在".to_string(),
            ).new_with_data(data),
            Error::ResourceExist (data)=> AuroraErrorInfo::new(
                20005,
                "resource already exists".to_string(),
                "资源已存在".to_string(),
            ).new_with_data(data),
            Error::ResourceSuffixNotSupportView (data)=> AuroraErrorInfo::new(
                20006,
                "resource suffix do not support online viewing".to_string(),
                "资源文件后缀不支持查看".to_string(),
            ).new_with_data(data),
            Error::ResourceSizeExceedLimit (data)=> AuroraErrorInfo::new(
                20007,
                "upload resource file size exceeds limit".to_string(),
                "上传资源文件大小超过限制".to_string(),
            ).new_with_data(data),
            Error::ResourceSuffixForbidChange (data)=> AuroraErrorInfo::new(
                20008,
                "resource suffix not allowed to be modified".to_string(),
                "资源文件后缀不支持修改".to_string(),
            ).new_with_data(data),
            Error::UdfResourceSuffixNotJar (data)=> AuroraErrorInfo::new(
                20009,
                "UDF resource suffix name must be jar".to_string(),
                "UDF资源文件后缀名只支持[jar]".to_string(),
            ).new_with_data(data),
            Error::HdfsCopyFail (data)=> AuroraErrorInfo::new(
                20010,
                "hdfs copy {0} -> {1} fail".to_string(),
                "hdfs复制失败：[{0}] -> [{1}]".to_string(),
            ).new_with_data(data),
            Error::ResourceFileExist (data)=> AuroraErrorInfo::new(
                20011,
                "resource file {0} already exists !".to_string(),
                "资源文件[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::ResourceFileNotExist (data)=> AuroraErrorInfo::new(
                20012,
                "resource file {0} not exists !".to_string(),
                "资源文件[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::UdfResourceIsBound (data)=> AuroraErrorInfo::new(
                20013,
                "udf resource file is bound by UDF functions:{0}".to_string(),
                "udf函数绑定了资源文件[{0}]".to_string(),
            ).new_with_data(data),
            Error::ResourceIsUsed (data)=> AuroraErrorInfo::new(
                20014,
                "resource file is used by process definition".to_string(),
                "资源文件被上线的流程定义使用了".to_string(),
            ).new_with_data(data),
            Error::ParentResourceNotExist (data)=> AuroraErrorInfo::new(
                20015,
                "parent resource not exist".to_string(),
                "父资源文件不存在".to_string(),
            ).new_with_data(data),
            Error::ResourceNotExistOrNoPermission (data)=> AuroraErrorInfo::new(
                20016,
                "resource not exist or no permission:please view the task node and remove error \
                 resource"
                    .to_string(),
                "请检查任务节点并移除无权限或者已删除的资源".to_string(),
            ).new_with_data(data),
            Error::ResourceIsAuthorized (data)=> AuroraErrorInfo::new(
                20017,
                "resource is authorized to user {0}:suffix not allowed to be modified".to_string(),
                "资源文件已授权其他用户[{0}]".to_string(),
            ).new_with_data(data),
            Error::UserNoOperationPerm (data)=> AuroraErrorInfo::new(
                30001,
                "user has no operation privilege".to_string(),
                "当前用户没有操作权限".to_string(),
            ).new_with_data(data),
            Error::UserNoOperationProjectPerm (data)=> AuroraErrorInfo::new(
                30002,
                "user {0} is not has project {1} permission".to_string(),
                "当前用户[{0}]没有[{1}]项目的操作权限".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceNotExist (data)=> AuroraErrorInfo::new(
                50001,
                "process instance {0} does not exist".to_string(),
                "工作流实例[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceExist (data)=> AuroraErrorInfo::new(
                50002,
                "process instance {0} already exists".to_string(),
                "工作流实例[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::ProcessDefineNotExist (data)=> AuroraErrorInfo::new(
                50003,
                "process definition {0} does not exist".to_string(),
                "工作流定义[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::ProcessDefineNotRelease (data)=> AuroraErrorInfo::new(
                50004,
                "process definition {0} process version {1} not online".to_string(),
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态".to_string(),
            ).new_with_data(data),
            Error::SubProcessDefineNotRelease (data)=> AuroraErrorInfo::new(
                50004,
                "exist sub process definition not online".to_string(),
                "存在子工作流定义不是上线状态".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceAlreadyChanged (data)=> AuroraErrorInfo::new(
                50005,
                "the status of process instance {0} is already {1}".to_string(),
                "工作流实例[{0}]的状态已经是[{1}]".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceStateOperationError (data)=> AuroraErrorInfo::new(
                50006,
                "the status of process instance {0} is {1}.to_string(),Cannot perform the \
                 operation"
                    .to_string(),
                "工作流实例[{0}]的状态是[{1}]，无法执行该操作".to_string(),
            ).new_with_data(data),
            Error::SubProcessInstanceNotExist (data)=> AuroraErrorInfo::new(
                50007,
                "the task belong to process instance does not exist".to_string(),
                "子工作流实例不存在".to_string(),
            ).new_with_data(data),
            Error::ProcessDefineNotAllowedEdit (data)=> AuroraErrorInfo::new(
                50008,
                "process definition {0} does not allow edit".to_string(),
                "工作流定义[{0}]不允许修改".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceExecutingCommand (data)=> AuroraErrorInfo::new(
                50009,
                "process instance {0} is executing command".to_string(),
                "工作流实例[{0}]正在执行命令".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceNotSubProcessInstance (data)=> AuroraErrorInfo::new(
                50010,
                "process instance {0} is not sub process instance".to_string(),
                "工作流实例[{0}]不是子工作流实例".to_string(),
            ).new_with_data(data),
            Error::TaskInstanceStateCountError (data)=> AuroraErrorInfo::new(
                50011,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            ).new_with_data(data),
            Error::CountProcessInstanceStateError (data)=> AuroraErrorInfo::new(
                50012,
                "count process instance state error".to_string(),
                "查询各状态流程实例数错误".to_string(),
            ).new_with_data(data),
            Error::CountProcessDefinitionUserError (data)=> AuroraErrorInfo::new(
                50013,
                "count process definition user error".to_string(),
                "查询各用户流程定义数错误".to_string(),
            ).new_with_data(data),
            Error::StartProcessInstanceError (data)=> AuroraErrorInfo::new(
                50014,
                "start process instance error".to_string(),
                "运行工作流实例错误".to_string(),
            ).new_with_data(data),
            Error::BatchStartProcessInstanceError (data)=> AuroraErrorInfo::new(
                50014,
                "batch start process instance error: {0}".to_string(),
                "批量运行工作流实例错误: {0}".to_string(),
            ).new_with_data(data),
            Error::ProcessInstanceError (data)=> AuroraErrorInfo::new(
                50014,
                "process instance delete error: {0}".to_string(),
                "工作流实例删除[{0}]错误".to_string(),
            ).new_with_data(data),
            Error::ExecuteProcessInstanceError (data)=> AuroraErrorInfo::new(
                50015,
                "execute process instance error".to_string(),
                "操作工作流实例错误".to_string(),
            ).new_with_data(data),
            Error::CheckProcessDefinitionError (data)=> AuroraErrorInfo::new(
                50016,
                "check process definition error".to_string(),
                "工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::QueryRecipientsAndCopyersByProcessDefinitionError (data)=> AuroraErrorInfo::new(
                50017,
                "query recipients and copyers by process definition error".to_string(),
                "查询收件人和抄送人错误".to_string(),
            ).new_with_data(data),
            Error::DataIsNotValid (data)=> AuroraErrorInfo::new(
                50017,
                "data {0} not valid".to_string(),
                "数据[{0}]无效".to_string(),
            ).new_with_data(data),
            Error::DataIsNull (data)=> AuroraErrorInfo::new(
                50018,
                "data {0} is null".to_string(),
                "数据[{0}]不能为空".to_string(),
            ).new_with_data(data),
            Error::ProcessNodeHasCycle (data)=> AuroraErrorInfo::new(
                50019,
                "process node has cycle".to_string(),
                "流程节点间存在循环依赖".to_string(),
            ).new_with_data(data),
            Error::ProcessNodeSParameterInvalid (data)=> AuroraErrorInfo::new(
                50020,
                "process node {0} parameter invalid".to_string(),
                "流程节点[{0}]参数无效".to_string(),
            ).new_with_data(data),
            Error::ProcessDefineStateOnline (data)=> AuroraErrorInfo::new(
                50021,
                "process definition [{0}] is already online".to_string(),
                "工作流定义[{0}]已上线".to_string(),
            ).new_with_data(data),
            Error::DeleteProcessDefineByCodeError (data)=> AuroraErrorInfo::new(
                50022,
                "delete process definition by code error".to_string(),
                "删除工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::ScheduleCronStateOnline (data)=> AuroraErrorInfo::new(
                50023,
                "the status of schedule {0} is already online".to_string(),
                "调度配置[{0}]已上线".to_string(),
            ).new_with_data(data),
            Error::DeleteScheduleCronByIdError (data)=> AuroraErrorInfo::new(
                50024,
                "delete schedule by id error".to_string(),
                "删除调度配置错误".to_string(),
            ).new_with_data(data),
            Error::BatchDeleteProcessDefineError (data)=> AuroraErrorInfo::new(
                50025,
                "batch delete process definition error".to_string(),
                "批量删除工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::BatchDeleteProcessDefineByCodesError (data)=> AuroraErrorInfo::new(
                50026,
                "batch delete process definition by codes {0} error".to_string(),
                "批量删除工作流定义[{0}]错误".to_string(),
            ).new_with_data(data),
            Error::DeleteProcessDefineByCodesError (data)=> AuroraErrorInfo::new(
                50026,
                "delete process definition by codes {0} error".to_string(),
                "删除工作流定义[{0}]错误".to_string(),
            ).new_with_data(data),
            Error::TenantNotSuitable (data)=> AuroraErrorInfo::new(
                50027,
                "there is not any tenant suitable: please choose a tenant available.".to_string(),
                "没有合适的租户，请选择可用的租户".to_string(),
            ).new_with_data(data),
            Error::ExportProcessDefineByIdError (data)=> AuroraErrorInfo::new(
                50028,
                "export process definition by id error".to_string(),
                "导出工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::BatchExportProcessDefineByIdsError (data)=> AuroraErrorInfo::new(
                50028,
                "batch export process definition by ids error".to_string(),
                "批量导出工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::ImportProcessDefineError (data)=> AuroraErrorInfo::new(
                50029,
                "import process definition error".to_string(),
                "导入工作流定义错误".to_string(),
            ).new_with_data(data),
            Error::TaskDefineNotExist (data)=> AuroraErrorInfo::new(
                50030,
                "task definition [{0}] does not exist".to_string(),
                "任务定义[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::CreateProcessTaskRelationError (data)=> AuroraErrorInfo::new(
                50032,
                "create process task relation error".to_string(),
                "创建工作流任务关系错误".to_string(),
            ).new_with_data(data),
            Error::ProcessTaskRelationNotExist (data)=> AuroraErrorInfo::new(
                50033,
                "process task relation [{0}] does not exist".to_string(),
                "工作流任务关系[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::ProcessTaskRelationExist (data)=> AuroraErrorInfo::new(
                50034,
                "process task relation is already exist  processCode:[{0}]".to_string(),
                "工作流任务关系已存在".to_string(),
            ).new_with_data(data),
            Error::ProcessDagIsEmpty (data)=> AuroraErrorInfo::new(
                50035,
                "process dag is empty".to_string(),
                "工作流dag是空".to_string(),
            ).new_with_data(data),
            Error::CheckProcessTaskRelationError (data)=> AuroraErrorInfo::new(
                50036,
                "check process task relation error".to_string(),
                "工作流任务关系参数错误".to_string(),
            ).new_with_data(data),
            Error::CreateTaskDefinitionError (data)=> AuroraErrorInfo::new(
                50037,
                "create task definition error".to_string(),
                "创建任务错误".to_string(),
            ).new_with_data(data),
            Error::UpdateTaskDefinitionError (data)=> AuroraErrorInfo::new(
                50038,
                "update task definition error".to_string(),
                "更新任务定义错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskDefinitionVersionsError (data)=> AuroraErrorInfo::new(
                50039,
                "query task definition versions error".to_string(),
                "查询任务历史版本信息出错".to_string(),
            ).new_with_data(data),
            Error::SwitchTaskDefinitionVersionError (data)=> AuroraErrorInfo::new(
                50040,
                "Switch task definition version error".to_string(),
                "切换任务版本出错".to_string(),
            ).new_with_data(data),
            Error::DeleteTaskDefinitionVersionError (data)=> AuroraErrorInfo::new(
                50041,
                "delete task definition version error".to_string(),
                "删除任务历史版本出错".to_string(),
            ).new_with_data(data),
            Error::DeleteTaskDefineByCodeError (data)=> AuroraErrorInfo::new(
                50042,
                "delete task definition by code error".to_string(),
                "删除任务定义错误".to_string(),
            ).new_with_data(data),
            Error::QueryDetailOfTaskDefinitionError (data)=> AuroraErrorInfo::new(
                50043,
                "query detail of task definition error".to_string(),
                "查询任务详细信息错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskDefinitionListPagingError (data)=> AuroraErrorInfo::new(
                50044,
                "query task definition list paging error".to_string(),
                "分页查询任务定义列表错误".to_string(),
            ).new_with_data(data),
            Error::TaskDefinitionNameExisted (data)=> AuroraErrorInfo::new(
                50045,
                "task definition name [{0}] already exists".to_string(),
                "任务定义名称[{0}]已经存在".to_string(),
            ).new_with_data(data),
            Error::ReleaseTaskDefinitionError (data)=> AuroraErrorInfo::new(
                50046,
                "release task definition error".to_string(),
                "上线任务错误".to_string(),
            ).new_with_data(data),
            Error::MoveProcessTaskRelationError (data)=> AuroraErrorInfo::new(
                50047,
                "move process task relation error".to_string(),
                "移动任务到其他工作流错误".to_string(),
            ).new_with_data(data),
            Error::DeleteTaskProcessRelationError (data)=> AuroraErrorInfo::new(
                50048,
                "delete process task relation error".to_string(),
                "删除工作流任务关系错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskProcessRelationError (data)=> AuroraErrorInfo::new(
                50049,
                "query process task relation error".to_string(),
                "查询工作流任务关系错误".to_string(),
            ).new_with_data(data),
            Error::TaskDefineStateOnline (data)=> AuroraErrorInfo::new(
                50050,
                "task definition [{0}] is already online".to_string(),
                "任务定义[{0}]已上线".to_string(),
            ).new_with_data(data),
            Error::TaskHasDownstream (data)=> AuroraErrorInfo::new(
                50051,
                "Task exists downstream [{0}] dependence".to_string(),
                "任务存在下游[{0}]依赖".to_string(),
            ).new_with_data(data),
            Error::TaskHasUpstream (data)=> AuroraErrorInfo::new(
                50052,
                "Task [{0}] exists upstream dependence".to_string(),
                "任务[{0}]存在上游依赖".to_string(),
            ).new_with_data(data),
            Error::MainTableUsingVersion (data)=> AuroraErrorInfo::new(
                50053,
                "the version that the master table is using".to_string(),
                "主表正在使用该版本".to_string(),
            ).new_with_data(data),
            Error::ProjectProcessNotMatch (data)=> AuroraErrorInfo::new(
                50054,
                "the project and the process is not match".to_string(),
                "项目和工作流不匹配".to_string(),
            ).new_with_data(data),
            Error::DeleteEdgeError (data)=> AuroraErrorInfo::new(
                50055,
                "delete edge error".to_string(),
                "删除工作流任务连接线错误".to_string(),
            ).new_with_data(data),
            Error::NotSupportUpdateTaskDefinition (data)=> AuroraErrorInfo::new(
                50056,
                "task state does not support modification".to_string(),
                "当前任务不支持修改".to_string(),
            ).new_with_data(data),
            Error::NotSupportCopyTaskType (data)=> AuroraErrorInfo::new(
                50057,
                "task type [{0}] does not support copy".to_string(),
                "不支持复制的任务类型[{0}]".to_string(),
            ).new_with_data(data),
            Error::HdfsNotStartup (data)=> AuroraErrorInfo::new(
                60001,
                "hdfs not startup".to_string(),
                "hdfs未启用".to_string(),
            ).new_with_data(data),
            Error::StorageNotStartup (data)=> AuroraErrorInfo::new(
                60002,
                "storage not startup".to_string(),
                "存储未启用".to_string(),
            ).new_with_data(data),
            Error::S3CannotRename (data)=> AuroraErrorInfo::new(
                60003,
                "directory cannot be renamed".to_string(),
                "S3无法重命名文件夹".to_string(),
            ).new_with_data(data),
            Error::QueryDatabaseStateError (data)=> AuroraErrorInfo::new(
                70001,
                "query database state error".to_string(),
                "查询数据库状态错误".to_string(),
            ).new_with_data(data),
            Error::CreateAccessTokenError (data)=> AuroraErrorInfo::new(
                70010,
                "create access token error".to_string(),
                "创建访问token错误".to_string(),
            ).new_with_data(data),
            Error::GenerateTokenError (data)=> AuroraErrorInfo::new(
                70011,
                "generate token error".to_string(),
                "生成token错误".to_string(),
            ).new_with_data(data),
            Error::QueryAccesstokenListPagingError (data)=> AuroraErrorInfo::new(
                70012,
                "query access token list paging error".to_string(),
                "分页查询访问token列表错误".to_string(),
            ).new_with_data(data),
            Error::UpdateAccessTokenError (data)=> AuroraErrorInfo::new(
                70013,
                "update access token error".to_string(),
                "更新访问token错误".to_string(),
            ).new_with_data(data),
            Error::DeleteAccessTokenError (data)=> AuroraErrorInfo::new(
                70014,
                "delete access token error".to_string(),
                "删除访问token错误".to_string(),
            ).new_with_data(data),
            Error::AccessTokenNotExist (data)=> AuroraErrorInfo::new(
                70015,
                "access token not exist".to_string(),
                "访问token不存在".to_string(),
            ).new_with_data(data),
            Error::QueryAccesstokenByUserError (data)=> AuroraErrorInfo::new(
                70016,
                "query access token by user error".to_string(),
                "查询访问指定用户的token错误".to_string(),
            ).new_with_data(data),
            Error::CommandStateCountError (data)=> AuroraErrorInfo::new(
                80001,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            ).new_with_data(data),
            Error::NegativeSizeNumberError (data)=> AuroraErrorInfo::new(
                80002,
                "query size number error".to_string(),
                "查询size错误".to_string(),
            ).new_with_data(data),
            Error::StartTimeBiggerThanEndTimeError (data)=> AuroraErrorInfo::new(
                80003,
                "start time bigger than end time error".to_string(),
                "开始时间在结束时间之后错误".to_string(),
            ).new_with_data(data),
            Error::QueueCountError (data)=> AuroraErrorInfo::new(
                90001,
                "queue count error".to_string(),
                "查询队列数据错误".to_string(),
            ).new_with_data(data),
            Error::KerberosStartupState (data)=> AuroraErrorInfo::new(
                100001,
                "get kerberos startup state error".to_string(),
                "获取kerberos启动状态错误".to_string(),
            ).new_with_data(data),
            Error::QueryAuditLogListPaging (data)=> AuroraErrorInfo::new(
                10057,
                "query audit log list paging".to_string(),
                "分页查询日志列表错误".to_string(),
            ).new_with_data(data),
            Error::PluginNotAUiComponent (data)=> AuroraErrorInfo::new(
                110001,
                "query plugin error: this plugin has no UI component".to_string(),
                "查询插件错误，此插件无UI组件".to_string(),
            ).new_with_data(data),
            Error::QueryPluginsResultIsNull (data)=> AuroraErrorInfo::new(
                110002,
                "query alarm plugins result is empty:please check the startup status of the alarm \
                 component and confirm that the relevant alarm plugin is successfully registered"
                    .to_string(),
                "查询告警插件为空".to_string(),
            ).new_with_data(data),
            Error::QueryPluginsError (data)=> AuroraErrorInfo::new(
                110003,
                "query plugins error".to_string(),
                "查询插件错误".to_string(),
            ).new_with_data(data),
            Error::QueryPluginDetailResultIsNull (data)=> AuroraErrorInfo::new(
                110004,
                "query plugin detail result is null".to_string(),
                "查询插件详情结果为空".to_string(),
            ).new_with_data(data),
            Error::UpdateAlertPluginInstanceError (data)=> AuroraErrorInfo::new(
                110005,
                "update alert plugin instance error".to_string(),
                "更新告警组和告警组插件实例错误".to_string(),
            ).new_with_data(data),
            Error::DeleteAlertPluginInstanceError (data)=> AuroraErrorInfo::new(
                110006,
                "delete alert plugin instance error".to_string(),
                "删除告警组和告警组插件实例错误".to_string(),
            ).new_with_data(data),
            Error::GetAlertPluginInstanceError (data)=> AuroraErrorInfo::new(
                110007,
                "get alert plugin instance error".to_string(),
                "获取告警组和告警组插件实例错误".to_string(),
            ).new_with_data(data),
            Error::CreateAlertPluginInstanceError (data)=> AuroraErrorInfo::new(
                110008,
                "create alert plugin instance error".to_string(),
                "创建告警组和告警组插件实例错误".to_string(),
            ).new_with_data(data),
            Error::QueryAllAlertPluginInstanceError (data)=> AuroraErrorInfo::new(
                110009,
                "query all alert plugin instance error".to_string(),
                "查询所有告警实例失败".to_string(),
            ).new_with_data(data),
            Error::PluginInstanceAlreadyExit (data)=> AuroraErrorInfo::new(
                110010,
                "plugin instance already exit".to_string(),
                "该告警插件实例已存在".to_string(),
            ).new_with_data(data),
            Error::ListPagingAlertPluginInstanceError (data)=> AuroraErrorInfo::new(
                110011,
                "query plugin instance page error".to_string(),
                "分页查询告警实例失败".to_string(),
            ).new_with_data(data),
            Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated (data)=> AuroraErrorInfo::new(
                110012,
                "failed to delete the alert instance there is an alarm group associated with this \
                 alert instance"
                    .to_string(),
                "删除告警实例失败，存在与此告警实例关联的警报组".to_string(),
            ).new_with_data(data),
            Error::ProcessDefinitionVersionIsUsed (data)=> AuroraErrorInfo::new(
                110013,
                "this process definition version is used".to_string(),
                "此工作流定义版本被使用".to_string(),
            ).new_with_data(data),
            Error::CreateEnvironmentError (data)=> AuroraErrorInfo::new(
                120001,
                "create environment error".to_string(),
                "创建环境失败".to_string(),
            ).new_with_data(data),
            Error::EnvironmentNameExists (data)=> AuroraErrorInfo::new(
                120002,
                "this environment name [{0}] already exists".to_string(),
                "环境名称[{0}]已经存在".to_string(),
            ).new_with_data(data),
            Error::EnvironmentNameIsNull (data)=> AuroraErrorInfo::new(
                120003,
                "this environment name shouldn't be empty.".to_string(),
                "环境名称不能为空".to_string(),
            ).new_with_data(data),
            Error::EnvironmentConfigIsNull (data)=> AuroraErrorInfo::new(
                120004,
                "this environment config shouldn't be empty.".to_string(),
                "环境配置信息不能为空".to_string(),
            ).new_with_data(data),
            Error::UpdateEnvironmentError (data)=> AuroraErrorInfo::new(
                120005,
                "update environment [{0}] info error".to_string(),
                "更新环境[{0}]信息失败".to_string(),
            ).new_with_data(data),
            Error::DeleteEnvironmentError (data)=> AuroraErrorInfo::new(
                120006,
                "delete environment error".to_string(),
                "删除环境信息失败".to_string(),
            ).new_with_data(data),
            Error::DeleteEnvironmentRelatedTaskExists (data)=> AuroraErrorInfo::new(
                120007,
                "delete environment error, related task exists".to_string(),
                "删除环境信息失败，存在关联任务".to_string(),
            ).new_with_data(data),
            Error::QueryEnvironmentByNameError (data)=> AuroraErrorInfo::new(
                1200008,
                "not found environment [{0}] ".to_string(),
                "查询环境名称[{0}]信息不存在".to_string(),
            ).new_with_data(data),
            Error::QueryEnvironmentByCodeError (data)=> AuroraErrorInfo::new(
                1200009,
                "not found environment [{0}] ".to_string(),
                "查询环境编码[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::QueryEnvironmentError (data)=> AuroraErrorInfo::new(
                1200010,
                "login user query environment error".to_string(),
                "分页查询环境列表错误".to_string(),
            ).new_with_data(data),
            Error::VerifyEnvironmentError (data)=> AuroraErrorInfo::new(
                1200011,
                "verify environment error".to_string(),
                "验证环境信息错误".to_string(),
            ).new_with_data(data),
            Error::GetRuleFormCreateJsonError (data)=> AuroraErrorInfo::new(
                1200012,
                "get rule form create json error".to_string(),
                "获取规则 FROM-CREATE-JSON 错误".to_string(),
            ).new_with_data(data),
            Error::QueryRuleListPagingError (data)=> AuroraErrorInfo::new(
                1200013,
                "query rule list paging error".to_string(),
                "获取规则分页列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryRuleListError (data)=> AuroraErrorInfo::new(
                1200014,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryRuleInputEntryListError (data)=> AuroraErrorInfo::new(
                1200015,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            ).new_with_data(data),
            Error::QueryExecuteResultListPagingError (data)=> AuroraErrorInfo::new(
                1200016,
                "query execute result list paging error".to_string(),
                "获取数据质量任务结果分页错误".to_string(),
            ).new_with_data(data),
            Error::GetDatasourceOptionsError (data)=> AuroraErrorInfo::new(
                1200017,
                "get datasource options error".to_string(),
                "获取数据源Options错误".to_string(),
            ).new_with_data(data),
            Error::GetDatasourceTablesError (data)=> AuroraErrorInfo::new(
                1200018,
                "get datasource tables error".to_string(),
                "获取数据源表列表错误".to_string(),
            ).new_with_data(data),
            Error::GetDatasourceTableColumnsError (data)=> AuroraErrorInfo::new(
                1200019,
                "get datasource table columns error".to_string(),
                "获取数据源表列名错误".to_string(),
            ).new_with_data(data),
            Error::TaskGroupNameExist (data)=> AuroraErrorInfo::new(
                130001,
                "this task group name is repeated in a project".to_string(),
                "该任务组名称在一个项目中已经使用".to_string(),
            ).new_with_data(data),
            Error::TaskGroupSizeError (data)=> AuroraErrorInfo::new(
                130002,
                "task group size error".to_string(),
                "任务组大小应该为大于1的整数".to_string(),
            ).new_with_data(data),
            Error::TaskGroupStatusError (data)=> AuroraErrorInfo::new(
                130003,
                "task group status error".to_string(),
                "任务组已经被关闭".to_string(),
            ).new_with_data(data),
            Error::TaskGroupFull (data)=> AuroraErrorInfo::new(
                130004,
                "task group is full".to_string(),
                "任务组已经满了".to_string(),
            ).new_with_data(data),
            Error::TaskGroupUsedSizeError (data)=> AuroraErrorInfo::new(
                130005,
                "the used size number of task group is dirty".to_string(),
                "任务组使用的容量发生了变化".to_string(),
            ).new_with_data(data),
            Error::TaskGroupQueueReleaseError (data)=> AuroraErrorInfo::new(
                130006,
                "failed to release task group queue".to_string(),
                "任务组资源释放时出现了错误".to_string(),
            ).new_with_data(data),
            Error::TaskGroupQueueAwakeError (data)=> AuroraErrorInfo::new(
                130007,
                "awake waiting task failed".to_string(),
                "任务组使唤醒等待任务时发生了错误".to_string(),
            ).new_with_data(data),
            Error::CreateTaskGroupError (data)=> AuroraErrorInfo::new(
                130008,
                "create task group error".to_string(),
                "创建任务组错误".to_string(),
            ).new_with_data(data),
            Error::UpdateTaskGroupError (data)=> AuroraErrorInfo::new(
                130009,
                "update task group list error".to_string(),
                "更新任务组错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskGroupListError (data)=> AuroraErrorInfo::new(
                130010,
                "query task group list error".to_string(),
                "查询任务组列表错误".to_string(),
            ).new_with_data(data),
            Error::CloseTaskGroupError (data)=> AuroraErrorInfo::new(
                130011,
                "close task group error".to_string(),
                "关闭任务组错误".to_string(),
            ).new_with_data(data),
            Error::StartTaskGroupError (data)=> AuroraErrorInfo::new(
                130012,
                "start task group error".to_string(),
                "启动任务组错误".to_string(),
            ).new_with_data(data),
            Error::QueryTaskGroupQueueListError (data)=> AuroraErrorInfo::new(
                130013,
                "query task group queue list error".to_string(),
                "查询任务组队列列表错误".to_string(),
            ).new_with_data(data),
            Error::TaskGroupCacheStartFailed (data)=> AuroraErrorInfo::new(
                130014,
                "cache start failed".to_string(),
                "任务组相关的缓存启动失败".to_string(),
            ).new_with_data(data),
            Error::EnvironmentWorkerGroupsIsInvalid (data)=> AuroraErrorInfo::new(
                130015,
                "environment worker groups is invalid format".to_string(),
                "环境关联的工作组参数解析错误".to_string(),
            ).new_with_data(data),
            Error::UpdateEnvironmentWorkerGroupRelationError (data)=> AuroraErrorInfo::new(
                130016,
                "update environment worker group relation error".to_string(),
                "更新环境关联的工作组错误".to_string(),
            ).new_with_data(data),
            Error::TaskGroupQueueAlreadyStart (data)=> AuroraErrorInfo::new(
                130017,
                "task group queue already start".to_string(),
                "节点已经获取任务组资源".to_string(),
            ).new_with_data(data),
            Error::TaskGroupStatusClosed (data)=> AuroraErrorInfo::new(
                130018,
                "The task group has been closed.".to_string(),
                "任务组已经被关闭".to_string(),
            ).new_with_data(data),
            Error::TaskGroupStatusOpened (data)=> AuroraErrorInfo::new(
                130019,
                "The task group has been opened.".to_string(),
                "任务组已经被开启".to_string(),
            ).new_with_data(data),
            Error::NotAllowToDisableOwnAccount (data)=> AuroraErrorInfo::new(
                130020,
                "Not allow to disable your own account".to_string(),
                "不能停用自己的账号".to_string(),
            ).new_with_data(data),
            Error::NotAllowToDeleteDefaultAlarmGroup (data)=> AuroraErrorInfo::new(
                130030,
                "Not allow to delete the default alarm group ".to_string(),
                "不能删除默认告警组".to_string(),
            ).new_with_data(data),
            Error::TimeZoneIllegal (data)=> AuroraErrorInfo::new(
                130031,
                "time zone [{0}] is illegal".to_string(),
                "时区参数 [{0}] 不合法".to_string(),
            ).new_with_data(data),
            Error::QueryK8sNamespaceListPagingError (data)=> AuroraErrorInfo::new(
                1300001,
                "login user query k8s namespace list paging error".to_string(),
                "分页查询k8s名称空间列表错误".to_string(),
            ).new_with_data(data),
            Error::K8sNamespaceExist (data)=> AuroraErrorInfo::new(
                1300002,
                "k8s namespace {0} already exists".to_string(),
                "k8s命名空间[{0}]已存在".to_string(),
            ).new_with_data(data),
            Error::CreateK8sNamespaceError (data)=> AuroraErrorInfo::new(
                1300003,
                "create k8s namespace error".to_string(),
                "创建k8s命名空间错误".to_string(),
            ).new_with_data(data),
            Error::UpdateK8sNamespaceError (data)=> AuroraErrorInfo::new(
                1300004,
                "update k8s namespace error".to_string(),
                "更新k8s命名空间信息错误".to_string(),
            ).new_with_data(data),
            Error::K8sNamespaceNotExist (data)=> AuroraErrorInfo::new(
                1300005,
                "k8s namespace {0} not exists".to_string(),
                "命名空间ID[{0}]不存在".to_string(),
            ).new_with_data(data),
            Error::K8sClientOpsError (data)=> AuroraErrorInfo::new(
                1300006,
                "k8s error with exception {0}".to_string(),
                "k8s操作报错[{0}]".to_string(),
            ).new_with_data(data),
            Error::VerifyK8sNamespaceError (data)=> AuroraErrorInfo::new(
                1300007,
                "verify k8s and namespace error".to_string(),
                "验证k8s命名空间信息错误".to_string(),
            ).new_with_data(data),
            Error::DeleteK8sNamespaceByIdError (data)=> AuroraErrorInfo::new(
                1300008,
                "delete k8s namespace by id error".to_string(),
                "删除命名空间错误".to_string(),
            ).new_with_data(data),
            Error::VerifyParameterNameFailed (data)=> AuroraErrorInfo::new(
                1300009,
                "The file name verify failed".to_string(),
                "文件命名校验失败".to_string(),
            ).new_with_data(data),
            Error::StoreOperateCreateError (data)=> AuroraErrorInfo::new(
                1300010,
                "create the resource failed".to_string(),
                "存储操作失败".to_string(),
            ).new_with_data(data),
            Error::GrantK8sNamespaceError (data)=> AuroraErrorInfo::new(
                1300011,
                "grant namespace error".to_string(),
                "授权资源错误".to_string(),
            ).new_with_data(data),
            Error::QueryUnauthorizedNamespaceError (data)=> AuroraErrorInfo::new(
                1300012,
                "query unauthorized namespace error".to_string(),
                "查询未授权命名空间错误".to_string(),
            ).new_with_data(data),
            Error::QueryAuthorizedNamespaceError (data)=> AuroraErrorInfo::new(
                1300013,
                "query authorized namespace error".to_string(),
                "查询授权命名空间错误".to_string(),
            ).new_with_data(data),
            Error::QueryCanUseK8sClusterError (data)=> AuroraErrorInfo::new(
                1300014,
                "login user query can used k8s cluster list error".to_string(),
                "查询可用k8s集群错误".to_string(),
            ).new_with_data(data),
            Error::ResourceFullNameTooLongError (data)=> AuroraErrorInfo::new(
                1300015,
                "resource's fullname is too long error".to_string(),
                "资源文件名过长".to_string(),
            ).new_with_data(data),
            Error::TenantFullNameTooLongError (data)=> AuroraErrorInfo::new(
                1300016,
                "tenant's fullname is too long error".to_string(),
                "租户名过长".to_string(),
            ).new_with_data(data),
        }
    }
}
