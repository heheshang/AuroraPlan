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
    SUCCESS(AuroraData, Option<Vec<String>>),
    InternalServerErrorArgs(AuroraData, Option<Vec<String>>),
    RequestParamsNotValidError(AuroraData, Option<Vec<String>>), //(10001, "request parameter {0} is not valid", "请求参数[{0}]无效"),
    TaskTimeoutParamsError(AuroraData, Option<Vec<String>>), //(10002, "task timeout parameter is not valid", "任务超时参数无效"),
    UserNameExist(AuroraData, Option<Vec<String>>),          //(10003, "user name already exists", "用户名已存在"),
    UserNameNull(AuroraData, Option<Vec<String>>),           //(10004, "user name is null", "用户名不能为空"),
    HdfsOperationError(AuroraData, Option<Vec<String>>),     //(10006, "hdfs operation error", "hdfs操作错误"),
    TaskInstanceNotFound(AuroraData, Option<Vec<String>>),   //(10008, "task instance not found", "任务实例不存在"),
    OsTenantCodeExist(AuroraData, Option<Vec<String>>), //(10009, "os tenant code {0} already exists", "操作系统租户[{0}]已存在"),
    UserNotExist(AuroraData, Option<Vec<String>>),      //(10010, "user {0} not exists", "用户[{0}]不存在"),
    AlertGroupNotExist(AuroraData, Option<Vec<String>>), //(10011, "alarm group not found", "告警组不存在"),
    AlertGroupExist(AuroraData, Option<Vec<String>>),   //(10012, "alarm group already exists", "告警组名称已存在"),
    UserNamePasswdError(AuroraData, Option<Vec<String>>), //(10013, "user name or password error", "用户名或密码错误"),
    LoginSessionFailed(AuroraData, Option<Vec<String>>), //(10014, "create session failed!", "创建session失败"),
    DatasourceExist(AuroraData, Option<Vec<String>>), //(10015, "data source name already exists", "数据源名称已存在"),
    DatasourceConnectFailed(AuroraData, Option<Vec<String>>), //(10016, "data source connection failed", "建立数据源连接失败"),
    TenantNotExist(AuroraData, Option<Vec<String>>),          //(10017, "tenant not exists", "租户不存在"),
    ProjectNotFound(AuroraData, Option<Vec<String>>),         //(10018, "project {0} not found ", "项目[{0}]不存在"),
    ProjectAlreadyExists(AuroraData, Option<Vec<String>>), //(10019, "project {0} already exists", "项目名称[{0}]已存在"),
    TaskInstanceNotExists(AuroraData, Option<Vec<String>>), //(10020, "task instance {0} does not exist", "任务实例[{0}]不存在"),
    TaskInstanceNotSubWorkflowInstance(AuroraData, Option<Vec<String>>), //(10021, "task instance {0} is not sub process instance", "任务实例[{0}]不是子流程实例"),
    ScheduleCronNotExists(AuroraData, Option<Vec<String>>), //(10022, "scheduler crontab {0} does not exist", "调度配置定时表达式[{0}]不存在"),
    ScheduleCronOnlineForbidUpdate(AuroraData, Option<Vec<String>>), //(10023, "online status does not allow update operations", "调度配置上线状态不允许修改"),
    ScheduleCronCheckFailed(AuroraData, Option<Vec<String>>), //(10024, "scheduler crontab expression validation failure: {0}", "调度配置定时表达式验证失败: {0}"),
    MasterNotExists(AuroraData, Option<Vec<String>>),         //(10025, "master does not exist", "无可用master节点"),
    ScheduleStatusUnknown(AuroraData, Option<Vec<String>>),   //(10026, "unknown status: {0}", "未知状态: {0}"),
    CreateAlertGroupError(AuroraData, Option<Vec<String>>),   //(10027, "create alert group error", "创建告警组错误"),
    QueryAllAlertgroupError(AuroraData, Option<Vec<String>>), //(10028, "query all alertgroup error", "查询告警组错误"),
    ListPagingAlertGroupError(AuroraData, Option<Vec<String>>), //(10029, "list paging alert group error", "分页查询告警组错误"),
    UpdateAlertGroupError(AuroraData, Option<Vec<String>>),     //(10030, "update alert group error", "更新告警组错误"),
    DeleteAlertGroupError(AuroraData, Option<Vec<String>>),     //(10031, "delete alert group error", "删除告警组错误"),
    AlertGroupGrantUserError(AuroraData, Option<Vec<String>>), //(10032, "alert group grant user error", "告警组授权用户错误"),
    CreateDatasourceError(AuroraData, Option<Vec<String>>),    //(10033, "create datasource error", "创建数据源错误"),
    UpdateDatasourceError(AuroraData, Option<Vec<String>>),    //(10034, "update datasource error", "更新数据源错误"),
    QueryDatasourceError(AuroraData, Option<Vec<String>>),     //(10035, "query datasource error", "查询数据源错误"),
    ConnectDatasourceFailure(AuroraData, Option<Vec<String>>), //(10036, "connect datasource failure", "建立数据源连接失败"),
    ConnectionTestFailure(AuroraData, Option<Vec<String>>), //(10037, "connection test failure", "测试数据源连接失败"),
    DeleteDataSourceFailure(AuroraData, Option<Vec<String>>), //(10038, "delete data source failure", "删除数据源失败"),
    VerifyDatasourceNameFailure(AuroraData, Option<Vec<String>>), //(10039, "verify datasource name failure", "验证数据源名称失败"),
    UnauthorizedDatasource(AuroraData, Option<Vec<String>>), //(10040, "unauthorized datasource", "未经授权的数据源"),
    AuthorizedDataSource(AuroraData, Option<Vec<String>>),   //(10041, "authorized data source", "授权数据源失败"),
    LoginSuccess(AuroraData, Option<Vec<String>>),           //(10042, "login success", "登录成功"),
    UserLoginFailure(AuroraData, Option<Vec<String>>),       //(10043, "user login failure", "用户登录失败"),
    ListWorkersError(AuroraData, Option<Vec<String>>),       //(10044, "list workers error", "查询worker列表错误"),
    ListMastersError(AuroraData, Option<Vec<String>>),       //(10045, "list masters error", "查询master列表错误"),
    UpdateProjectError(AuroraData, Option<Vec<String>>),     //(10046, "update project error", "更新项目信息错误"),
    QueryProjectDetailsByCodeError(AuroraData, Option<Vec<String>>), //(10047, "query project details by code error", "查询项目详细信息错误"),
    CreateProjectError(AuroraData, Option<Vec<String>>),             //(10048, "create project error", "创建项目错误"),
    LoginUserQueryProjectListPagingError(AuroraData, Option<Vec<String>>), //(10049, "login user query project list paging error", "分页查询项目列表错误"),
    DeleteProjectError(AuroraData, Option<Vec<String>>), //(10050, "delete project error", "删除项目错误"),
    QueryUnauthorizedProjectError(AuroraData, Option<Vec<String>>), //(10051, "query unauthorized project error", "查询未授权项目错误"),
    QueryAuthorizedProject(AuroraData, Option<Vec<String>>), //(10052, "query authorized project", "查询授权项目错误"),
    QueryQueueListError(AuroraData, Option<Vec<String>>),    //(10053, "query queue list error", "查询队列列表错误"),
    CreateResourceError(AuroraData, Option<Vec<String>>),    //(10054, "create resource error", "创建资源错误"),
    UpdateResourceError(AuroraData, Option<Vec<String>>),    //(10055, "update resource error", "更新资源错误"),
    QueryResourcesListError(AuroraData, Option<Vec<String>>), //(10056, "query resources list error", "查询资源列表错误"),
    QueryResourcesListPaging(AuroraData, Option<Vec<String>>), //(10057, "query resources list paging", "分页查询资源列表错误"),
    DeleteResourceError(AuroraData, Option<Vec<String>>),      //(10058, "delete resource error", "删除资源错误"),
    VerifyResourceByNameAndTypeError(AuroraData, Option<Vec<String>>), //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
    ViewResourceFileOnLineError(AuroraData, Option<Vec<String>>), //(10060, "view resource file online error", "查看资源文件错误"),
    CreateResourceFileOnLineError(AuroraData, Option<Vec<String>>), //(10061, "create resource file online error", "创建资源文件错误"),
    ResourceFileIsEmpty(AuroraData, Option<Vec<String>>), //(10062, "resource file is empty", "资源文件内容不能为空"),
    EditResourceFileOnLineError(AuroraData, Option<Vec<String>>), //(10063, "edit resource file online error", "更新资源文件错误"),
    DownloadResourceFileError(AuroraData, Option<Vec<String>>), //(10064, "download resource file error", "下载资源文件错误"),
    CreateUdfFunctionError(AuroraData, Option<Vec<String>>), //(10065, "create udf function error", "创建UDF函数错误"),
    ViewUdfFunctionError(AuroraData, Option<Vec<String>>),   //(10066, "view udf function error", "查询UDF函数错误"),
    UpdateUdfFunctionError(AuroraData, Option<Vec<String>>), //(10067, "update udf function error", "更新UDF函数错误"),
    QueryUdfFunctionListPagingError(AuroraData, Option<Vec<String>>), //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
    QueryDatasourceByTypeError(AuroraData, Option<Vec<String>>), //(10069, "query datasource by type error", "查询数据源信息错误"),
    VerifyUdfFunctionNameError(AuroraData, Option<Vec<String>>), //(10070, "verify udf function name error", "UDF函数名称验证错误"),
    DeleteUdfFunctionError(AuroraData, Option<Vec<String>>), //(10071, "delete udf function error", "删除UDF函数错误"),
    AuthorizedFileResourceError(AuroraData, Option<Vec<String>>), //(10072, "authorized file resource error", "授权资源文件错误"),
    AuthorizeResourceTree(AuroraData, Option<Vec<String>>), //(10073, "authorize resource tree display error", "授权资源目录树错误"),
    UnauthorizedUdfFunctionError(AuroraData, Option<Vec<String>>), //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
    AuthorizedUdfFunctionError(AuroraData, Option<Vec<String>>), //(10075, "authorized udf function error", "授权UDF函数错误"),
    CreateScheduleError(AuroraData, Option<Vec<String>>),        //(10076, "create schedule error", "创建调度配置错误"),
    UpdateScheduleError(AuroraData, Option<Vec<String>>),        //(10077, "update schedule error", "更新调度配置错误"),
    PublishScheduleOnlineError(AuroraData, Option<Vec<String>>), //(10078, "publish schedule online error", "上线调度配置错误"),
    OfflineScheduleError(AuroraData, Option<Vec<String>>), //(10079, "offline schedule error", "下线调度配置错误"),
    QueryScheduleListPagingError(AuroraData, Option<Vec<String>>), //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
    QueryScheduleListError(AuroraData, Option<Vec<String>>), //(10081, "query schedule list error", "查询调度配置列表错误"),
    QueryTaskListPagingError(AuroraData, Option<Vec<String>>), //(10082, "query task list paging error", "分页查询任务列表错误"),
    QueryTaskRecordListPagingError(AuroraData, Option<Vec<String>>), //(10083, "query task record list paging error", "分页查询任务记录错误"),
    CreateTenantError(AuroraData, Option<Vec<String>>),              //(10084, "create tenant error", "创建租户错误"),
    QueryTenantListPagingError(AuroraData, Option<Vec<String>>), //(10085, "query tenant list paging error", "分页查询租户列表错误"),
    QueryTenantListError(AuroraData, Option<Vec<String>>), //(10086, "query tenant list error", "查询租户列表错误"),
    UpdateTenantError(AuroraData, Option<Vec<String>>),    //(10087, "update tenant error", "更新租户错误"),
    DeleteTenantByIdError(AuroraData, Option<Vec<String>>), //(10088, "delete tenant by id error", "删除租户错误"),
    VerifyOsTenantCodeError(AuroraData, Option<Vec<String>>), //(10089, "verify os tenant code error", "操作系统租户验证错误"),
    CreateUserError(AuroraData, Option<Vec<String>>),         //(10090, "create user error", "创建用户错误"),
    QueryUserListPagingError(AuroraData, Option<Vec<String>>), //(10091, "query user list paging error", "分页查询用户列表错误"),
    UpdateUserError(AuroraData, Option<Vec<String>>),          //(10092, "update user error", "更新用户错误"),
    DeleteUserByIdError(AuroraData, Option<Vec<String>>),      //(10093, "delete user by id error", "删除用户错误"),
    GrantProjectError(AuroraData, Option<Vec<String>>),        //(10094, "grant project error", "授权项目错误"),
    GrantResourceError(AuroraData, Option<Vec<String>>),       //(10095, "grant resource error", "授权资源错误"),
    GrantUdfFunctionError(AuroraData, Option<Vec<String>>),    //(10096, "grant udf function error", "授权UDF函数错误"),
    GrantDatasourceError(AuroraData, Option<Vec<String>>),     //(10097, "grant datasource error", "授权数据源错误"),
    GetUserInfoError(AuroraData, Option<Vec<String>>),         //(10098, "get user info error", "获取用户信息错误"),
    UserListError(AuroraData, Option<Vec<String>>),            //(10099, "user list error", "查询用户列表错误"),
    VerifyUsernameError(AuroraData, Option<Vec<String>>),      //(10100, "verify username error", "用户名验证错误"),
    UnauthorizedUserError(AuroraData, Option<Vec<String>>), //(10101, "unauthorized user error", "查询未授权用户错误"),
    AuthorizedUserError(AuroraData, Option<Vec<String>>),   //(10102, "authorized user error", "查询授权用户错误"),
    QueryTaskInstanceLogError(AuroraData, Option<Vec<String>>), //(10103, "view task instance log error", "查询任务实例日志错误"),
    DownloadTaskInstanceLogFileError(AuroraData, Option<Vec<String>>), //(10104, "download task instance log file error", "下载任务日志文件错误"),
    CreateProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10105, "create process definition error", "创建工作流错误"),
    VerifyProcessDefinitionNameUniqueError(AuroraData, Option<Vec<String>>), //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
    UpdateProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10107, "update process definition error", "更新工作流定义错误"),
    ReleaseProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10108, "release process definition error", "上线工作流错误"),
    QueryDetailOfProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
    QueryProcessDefinitionList(AuroraData, Option<Vec<String>>), //(10110, "query process definition list", "查询工作流列表错误"),
    EncapsulationTreeviewStructureError(AuroraData, Option<Vec<String>>), //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
    GetTasksListByProcessDefinitionIdError(AuroraData, Option<Vec<String>>), //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
    QueryProcessInstanceListPagingError(AuroraData, Option<Vec<String>>), //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
    QueryTaskListByProcessInstanceIdError(AuroraData, Option<Vec<String>>), //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
    UpdateProcessInstanceError(AuroraData, Option<Vec<String>>), //(10115, "update process instance error", "更新工作流实例错误"),
    QueryProcessInstanceByIdError(AuroraData, Option<Vec<String>>), //(10116, "query process instance by id error", "查询工作流实例错误"),
    DeleteProcessInstanceByIdError(AuroraData, Option<Vec<String>>), //(10117, "delete process instance by id error", "删除工作流实例错误"),
    QuerySubProcessInstanceDetailInfoByTaskIdError(AuroraData, Option<Vec<String>>), //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
    QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(AuroraData, Option<Vec<String>>), //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
    QueryProcessInstanceAllVariablesError(AuroraData, Option<Vec<String>>), //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
    EncapsulationProcessInstanceGanttStructureError(AuroraData, Option<Vec<String>>), //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
    QueryProcessDefinitionListPagingError(AuroraData, Option<Vec<String>>), //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
    SignOutError(AuroraData, Option<Vec<String>>),                          //(10123, "sign out error", "退出错误"),
    OsTenantCodeHasAlreadyExists(AuroraData, Option<Vec<String>>), //(10124, "os tenant code has already exists", "操作系统租户已存在"),
    IpIsEmpty(AuroraData, Option<Vec<String>>),                    //(10125, "ip is empty", "IP地址不能为空"),
    ScheduleCronReleaseNeedNotChange(AuroraData, Option<Vec<String>>), //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
    CreateQueueError(AuroraData, Option<Vec<String>>),                 //(10127, "create queue error", "创建队列错误"),
    QueueNotExist(AuroraData, Option<Vec<String>>), //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
    QueueValueExist(AuroraData, Option<Vec<String>>), //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
    QueueNameExist(AuroraData, ErrorParam),         //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
    UpdateQueueError(AuroraData, Option<Vec<String>>), //(10131, "update queue error", "更新队列信息错误"),
    NeedNotUpdateQueue(AuroraData, Option<Vec<String>>), //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
    VerifyQueueError(AuroraData, Option<Vec<String>>),   //(10133, "verify queue error", "验证队列信息错误"),
    NameNull(AuroraData, Option<Vec<String>>),           //(10134, "name must be not null", "名称不能为空"),
    NameExist(AuroraData, Option<Vec<String>>),          //(10135, "name {0} already exists", "名称[{0}]已存在"),
    SaveError(AuroraData, Option<Vec<String>>),          //(10136, "save error", "保存错误"),
    DeleteProjectErrorDefinesNotNull(AuroraData, Option<Vec<String>>), //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
    BatchDeleteProcessInstanceByIdsError(AuroraData, Option<Vec<String>>), //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
    PreviewScheduleError(AuroraData, Option<Vec<String>>), //(10139, "preview schedule error", "预览调度配置错误"),
    ParseToCronExpressionError(AuroraData, Option<Vec<String>>), //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
    ScheduleStartTimeEndTimeSame(AuroraData, Option<Vec<String>>), //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
    DeleteTenantByIdFail(AuroraData, Option<Vec<String>>), //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
    DeleteTenantByIdFailDefines(AuroraData, Option<Vec<String>>), //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
    DeleteTenantByIdFailUsers(AuroraData, Option<Vec<String>>), //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
    DeleteWorkerGroupByIdFail(AuroraData, Option<Vec<String>>), //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
    QueryWorkerGroupFail(AuroraData, Option<Vec<String>>), //(10146, "query worker group fail ", "查询worker分组失败"),
    DeleteWorkerGroupFail(AuroraData, Option<Vec<String>>), //(10147, "delete worker group fail ", "删除worker分组失败"),
    UserDisabled(AuroraData, Option<Vec<String>>),          //(10148, "The current user is disabled", "当前用户已停用"),
    CopyProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
    MoveProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
    SwitchProcessDefinitionVersionError(AuroraData, Option<Vec<String>>), //(10151, "Switch process definition version error", "切换工作流版本出错"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(AuroraData, Option<Vec<String>>), //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
    QueryProcessDefinitionVersionsError(AuroraData, Option<Vec<String>>), //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
    DeleteProcessDefinitionVersionError(AuroraData, Option<Vec<String>>), //(10156, "delete process definition version error", "删除工作流历史版本出错"),

    QueryUserCreatedProjectError(AuroraData, Option<Vec<String>>), //(10157, "query user created project error error", "查询用户创建的项目错误"),
    ProcessDefinitionCodesIsEmpty(AuroraData, Option<Vec<String>>), //(10158, "process definition codes is empty", "工作流CODES不能为空"),
    BatchCopyProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10159, "batch copy process definition error", "复制工作流错误"),
    BatchMoveProcessDefinitionError(AuroraData, Option<Vec<String>>), //(10160, "batch move process definition error", "移动工作流错误"),
    QueryWorkflowLineageError(AuroraData, Option<Vec<String>>), //(10161, "query workflow lineage error", "查询血缘失败"),
    QueryAuthorizedAndUserCreatedProjectError(AuroraData, Option<Vec<String>>), //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
    DeleteProcessDefinitionByCodeFail(AuroraData, Option<Vec<String>>), //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
    CheckOsTenantCodeError(AuroraData, Option<Vec<String>>), //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
    ForceTaskSuccessError(AuroraData, Option<Vec<String>>), //(10165, "force task success error", "强制成功任务实例错误"),
    TaskInstanceStateOperationError(AuroraData, Option<Vec<String>>), //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
    DatasourceTypeNotExist(AuroraData, Option<Vec<String>>), //(10167, "data source type not exist", "数据源类型不存在"),
    ProcessDefinitionNameExist(AuroraData, Option<Vec<String>>), //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
    DatasourceDbTypeIllegal(AuroraData, Option<Vec<String>>), //(10169, "datasource type illegal", "数据源类型参数不合法"),
    DatasourcePortIllegal(AuroraData, Option<Vec<String>>), //(10170, "datasource port illegal", "数据源端口参数不合法"),
    DatasourceOtherParamsIllegal(AuroraData, Option<Vec<String>>), //(10171, "datasource other params illegal", "数据源其他参数不合法"),
    DatasourceNameIllegal(AuroraData, Option<Vec<String>>), //(10172, "datasource name illegal", "数据源名称不合法"),
    DatasourceHostIllegal(AuroraData, Option<Vec<String>>), //(10173, "datasource host illegal", "数据源HOST不合法"),
    DeleteWorkerGroupNotExist(AuroraData, Option<Vec<String>>), //(10174, "delete worker group not exist ", "删除worker分组不存在"),
    CreateWorkerGroupForbiddenInDocker(AuroraData, Option<Vec<String>>), //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
    DeleteWorkerGroupForbiddenInDocker(AuroraData, Option<Vec<String>>), //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
    WorkerAddressInvalid(AuroraData, Option<Vec<String>>), //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
    QueryWorkerAddressListFail(AuroraData, Option<Vec<String>>), //(10178, "query worker address list fail ", "查询worker地址列表失败"),
    TransformProjectOwnership(AuroraData, Option<Vec<String>>), //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
    QueryAlertGroupError(AuroraData, Option<Vec<String>>),      //(10180, "query alert group error", "查询告警组错误"),
    CurrentLoginUserTenantNotExist(AuroraData, Option<Vec<String>>), //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
    RevokeProjectError(AuroraData, Option<Vec<String>>), //(10182, "revoke project error", "撤销项目授权错误"),
    QueryAuthorizedUser(AuroraData, Option<Vec<String>>), //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
    ProjectNotExist(AuroraData, Option<Vec<String>>), //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
    TaskInstanceHostIsNull(AuroraData, Option<Vec<String>>), //(10191, "task instance host is null", "任务实例host为空"),
    QueryExecutingWorkflowError(AuroraData, Option<Vec<String>>), //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

    UdfFunctionNotExist(AuroraData, Option<Vec<String>>), //(20001, "UDF function not found", "UDF函数不存在"),
    UdfFunctionExists(AuroraData, Option<Vec<String>>),   //(20002, "UDF function already exists", "UDF函数已存在"),
    ResourceNotExist(AuroraData, Option<Vec<String>>),    //(20004, "resource not exist", "资源不存在"),
    ResourceExist(AuroraData, Option<Vec<String>>),       //(20005, "resource already exists", "资源已存在"),
    ResourceSuffixNotSupportView(AuroraData, Option<Vec<String>>), //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
    ResourceSizeExceedLimit(AuroraData, Option<Vec<String>>), //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
    ResourceSuffixForbidChange(AuroraData, Option<Vec<String>>), //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
    UdfResourceSuffixNotJar(AuroraData, Option<Vec<String>>), //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
    HdfsCopyFail(AuroraData, Option<Vec<String>>), //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
    ResourceFileExist(AuroraData, Option<Vec<String>>), //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
    ResourceFileNotExist(AuroraData, Option<Vec<String>>), //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
    UdfResourceIsBound(AuroraData, Option<Vec<String>>), //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
    ResourceIsUsed(AuroraData, Option<Vec<String>>), //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
    ParentResourceNotExist(AuroraData, Option<Vec<String>>), //(20015, "parent resource not exist", "父资源文件不存在"),
    ResourceNotExistOrNoPermission(AuroraData, Option<Vec<String>>), //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
    ResourceIsAuthorized(AuroraData, Option<Vec<String>>), //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

    UserNoOperationPerm(AuroraData, Option<Vec<String>>), //(30001, "user has no operation privilege", "当前用户没有操作权限"),
    UserNoOperationProjectPerm(AuroraData, Option<Vec<String>>), //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

    ProcessInstanceNotExist(AuroraData, Option<Vec<String>>), //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
    ProcessInstanceExist(AuroraData, Option<Vec<String>>), //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
    ProcessDefineNotExist(AuroraData, Option<Vec<String>>), //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
    ProcessDefineNotRelease(AuroraData, Option<Vec<String>>), //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
    SubProcessDefineNotRelease(AuroraData, Option<Vec<String>>), //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
    ProcessInstanceAlreadyChanged(AuroraData, Option<Vec<String>>), //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
    ProcessInstanceStateOperationError(AuroraData, Option<Vec<String>>), //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
    SubProcessInstanceNotExist(AuroraData, Option<Vec<String>>), //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
    ProcessDefineNotAllowedEdit(AuroraData, Option<Vec<String>>), //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
    ProcessInstanceExecutingCommand(AuroraData, Option<Vec<String>>), //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
    ProcessInstanceNotSubProcessInstance(AuroraData, Option<Vec<String>>), //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
    TaskInstanceStateCountError(AuroraData, Option<Vec<String>>), //(50011, "task instance state count error", "查询各状态任务实例数错误"),
    CountProcessInstanceStateError(AuroraData, Option<Vec<String>>), //(50012, "count process instance state error", "查询各状态流程实例数错误"),
    CountProcessDefinitionUserError(AuroraData, Option<Vec<String>>), //(50013, "count process definition user error", "查询各用户流程定义数错误"),
    StartProcessInstanceError(AuroraData, Option<Vec<String>>), //(50014, "start process instance error", "运行工作流实例错误"),
    BatchStartProcessInstanceError(AuroraData, Option<Vec<String>>), //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
    ProcessInstanceError(AuroraData, Option<Vec<String>>), //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
    ExecuteProcessInstanceError(AuroraData, Option<Vec<String>>), //(50015, "execute process instance error", "操作工作流实例错误"),
    CheckProcessDefinitionError(AuroraData, Option<Vec<String>>), //(50016, "check process definition error", "工作流定义错误"),
    QueryRecipientsAndCopyersByProcessDefinitionError(AuroraData, Option<Vec<String>>), //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误"),
    DataIsNotValid(AuroraData, Option<Vec<String>>), //(50017, "data {0} not valid", "数据[{0}]无效"),
    DataIsNull(AuroraData, Option<Vec<String>>),     //(50018, "data {0} is null", "数据[{0}]不能为空"),
    ProcessNodeHasCycle(AuroraData, Option<Vec<String>>), //(50019, "process node has cycle", "流程节点间存在循环依赖"),
    ProcessNodeSParameterInvalid(AuroraData, Option<Vec<String>>), //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效"),
    ProcessDefineStateOnline(AuroraData, Option<Vec<String>>), //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线"),
    DeleteProcessDefineByCodeError(AuroraData, Option<Vec<String>>), //(50022, "delete process definition by code error", "删除工作流定义错误"),
    ScheduleCronStateOnline(AuroraData, Option<Vec<String>>), //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线"),
    DeleteScheduleCronByIdError(AuroraData, Option<Vec<String>>), //(50024, "delete schedule by id error", "删除调度配置错误"),
    BatchDeleteProcessDefineError(AuroraData, Option<Vec<String>>), //(50025, "batch delete process definition error", "批量删除工作流定义错误"),
    BatchDeleteProcessDefineByCodesError(AuroraData, Option<Vec<String>>), //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误"),
    DeleteProcessDefineByCodesError(AuroraData, Option<Vec<String>>), //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误"),
    TenantNotSuitable(AuroraData, Option<Vec<String>>), //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户"),
    ExportProcessDefineByIdError(AuroraData, Option<Vec<String>>), //(50028, "export process definition by id error", "导出工作流定义错误"),
    BatchExportProcessDefineByIdsError(AuroraData, Option<Vec<String>>), //(50028, "batch export process definition by ids error", "批量导出工作流定义错误"),
    ImportProcessDefineError(AuroraData, Option<Vec<String>>), //(50029, "import process definition error", "导入工作流定义错误"),
    TaskDefineNotExist(AuroraData, Option<Vec<String>>), //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在"),
    CreateProcessTaskRelationError(AuroraData, Option<Vec<String>>), //(50032, "create process task relation error", "创建工作流任务关系错误"),
    ProcessTaskRelationNotExist(AuroraData, Option<Vec<String>>), //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在"),
    ProcessTaskRelationExist(AuroraData, Option<Vec<String>>), //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]"),
    ProcessDagIsEmpty(AuroraData, Option<Vec<String>>),        //(50035, "process dag is empty", "工作流dag是空"),
    CheckProcessTaskRelationError(AuroraData, Option<Vec<String>>), //(50036, "check process task relation error", "工作流任务关系参数错误"),
    CreateTaskDefinitionError(AuroraData, Option<Vec<String>>), //(50037, "create task definition error", "创建任务错误"),
    UpdateTaskDefinitionError(AuroraData, Option<Vec<String>>), //(50038, "update task definition error", "更新任务定义错误"),
    QueryTaskDefinitionVersionsError(AuroraData, Option<Vec<String>>), //(50039, "query task definition versions error", "查询任务历史版本信息出错"),
    SwitchTaskDefinitionVersionError(AuroraData, Option<Vec<String>>), //(50040, "Switch task definition version error", "切换任务版本出错"),
    DeleteTaskDefinitionVersionError(AuroraData, Option<Vec<String>>), //(50041, "delete task definition version error", "删除任务历史版本出错"),
    DeleteTaskDefineByCodeError(AuroraData, Option<Vec<String>>), //(50042, "delete task definition by code error", "删除任务定义错误"),
    QueryDetailOfTaskDefinitionError(AuroraData, Option<Vec<String>>), //(50043, "query detail of task definition error", "查询任务详细信息错误"),
    QueryTaskDefinitionListPagingError(AuroraData, Option<Vec<String>>), //(50044, "query task definition list paging error", "分页查询任务定义列表错误"),
    TaskDefinitionNameExisted(AuroraData, Option<Vec<String>>), //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在"),
    ReleaseTaskDefinitionError(AuroraData, Option<Vec<String>>), //(50046, "release task definition error", "上线任务错误"),
    MoveProcessTaskRelationError(AuroraData, Option<Vec<String>>), //(50047, "move process task relation error", "移动任务到其他工作流错误"),
    DeleteTaskProcessRelationError(AuroraData, Option<Vec<String>>), //(50048, "delete process task relation error", "删除工作流任务关系错误"),
    QueryTaskProcessRelationError(AuroraData, Option<Vec<String>>), //(50049, "query process task relation error", "查询工作流任务关系错误"),
    TaskDefineStateOnline(AuroraData, Option<Vec<String>>), //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线"),
    TaskHasDownstream(AuroraData, Option<Vec<String>>), //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖"),
    TaskHasUpstream(AuroraData, Option<Vec<String>>), //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖"),
    MainTableUsingVersion(AuroraData, Option<Vec<String>>), //(50053, "the version that the master table is using", "主表正在使用该版本"),
    ProjectProcessNotMatch(AuroraData, Option<Vec<String>>), //(50054, "the project and the process is not match", "项目和工作流不匹配"),
    DeleteEdgeError(AuroraData, Option<Vec<String>>),        //(50055, "delete edge error", "删除工作流任务连接线错误"),
    NotSupportUpdateTaskDefinition(AuroraData, Option<Vec<String>>), //(50056, "task state does not support modification", "当前任务不支持修改"),
    NotSupportCopyTaskType(AuroraData, Option<Vec<String>>), //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]"),
    HdfsNotStartup(AuroraData, Option<Vec<String>>),         //(60001, "hdfs not startup", "hdfs未启用"),
    StorageNotStartup(AuroraData, Option<Vec<String>>),      //(60002, "storage not startup", "存储未启用"),
    S3CannotRename(AuroraData, Option<Vec<String>>), //(60003, "directory cannot be renamed", "S3无法重命名文件夹"),
    /**
     * for monitor
     */
    QueryDatabaseStateError(AuroraData, Option<Vec<String>>), //(70001, "query database state error", "查询数据库状态错误"),

    CreateAccessTokenError(AuroraData, Option<Vec<String>>), //(70010, "create access token error", "创建访问token错误"),
    GenerateTokenError(AuroraData, Option<Vec<String>>),     //(70011, "generate token error", "生成token错误"),
    QueryAccesstokenListPagingError(AuroraData, Option<Vec<String>>), //(70012, "query access token list paging error", "分页查询访问token列表错误"),
    UpdateAccessTokenError(AuroraData, Option<Vec<String>>), //(70013, "update access token error", "更新访问token错误"),
    DeleteAccessTokenError(AuroraData, Option<Vec<String>>), //(70014, "delete access token error", "删除访问token错误"),
    AccessTokenNotExist(AuroraData, Option<Vec<String>>),    //(70015, "access token not exist", "访问token不存在"),
    QueryAccesstokenByUserError(AuroraData, Option<Vec<String>>), //(70016, "query access token by user error", "查询访问指定用户的token错误"),

    CommandStateCountError(AuroraData, Option<Vec<String>>), //(80001, "task instance state count error", "查询各状态任务实例数错误"),
    NegativeSizeNumberError(AuroraData, Option<Vec<String>>), //(80002, "query size number error", "查询size错误"),
    StartTimeBiggerThanEndTimeError(AuroraData, Option<Vec<String>>), //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误"),
    QueueCountError(AuroraData, Option<Vec<String>>), //(90001, "queue count error", "查询队列数据错误"),

    KerberosStartupState(AuroraData, Option<Vec<String>>), //(100001, "get kerberos startup state error", "获取kerberos启动状态错误"),

    // audit log
    QueryAuditLogListPaging(AuroraData, Option<Vec<String>>), //(10057, "query resources list paging", "分页查询资源列表错误"),

    //plugin
    PluginNotAUiComponent(AuroraData, Option<Vec<String>>), //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件"),
    QueryPluginsResultIsNull(AuroraData, Option<Vec<String>>), //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功"),
    QueryPluginsError(AuroraData, Option<Vec<String>>),        //(110003, "query plugins error", "查询插件错误"),
    QueryPluginDetailResultIsNull(AuroraData, Option<Vec<String>>), //(110004, "query plugin detail result is null", "查询插件详情结果为空"),

    UpdateAlertPluginInstanceError(AuroraData, Option<Vec<String>>), //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误"),
    DeleteAlertPluginInstanceError(AuroraData, Option<Vec<String>>), //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误"),
    GetAlertPluginInstanceError(AuroraData, Option<Vec<String>>), //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误"),
    CreateAlertPluginInstanceError(AuroraData, Option<Vec<String>>), //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误"),
    QueryAllAlertPluginInstanceError(AuroraData, Option<Vec<String>>), //(110009, "query all alert plugin instance error", "查询所有告警实例失败"),
    PluginInstanceAlreadyExit(AuroraData, Option<Vec<String>>), //(110010, "plugin instance already exit", "该告警插件实例已存在"),
    ListPagingAlertPluginInstanceError(AuroraData, Option<Vec<String>>), //(110011, "query plugin instance page error", "分页查询告警实例失败"),
    DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(AuroraData, Option<Vec<String>>), //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组"),
    ProcessDefinitionVersionIsUsed(AuroraData, Option<Vec<String>>), //(110013, "this process definition version is used", "此工作流定义版本被使用"),

    CreateEnvironmentError(AuroraData, Option<Vec<String>>), //(120001, "create environment error", "创建环境失败"),
    EnvironmentNameExists(AuroraData, Option<Vec<String>>), //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在"),
    EnvironmentNameIsNull(AuroraData, Option<Vec<String>>), //(120003, "this environment name shouldn't be empty.", "环境名称不能为空"),
    EnvironmentConfigIsNull(AuroraData, Option<Vec<String>>), //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空"),
    UpdateEnvironmentError(AuroraData, Option<Vec<String>>), //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败"),
    DeleteEnvironmentError(AuroraData, Option<Vec<String>>), //(120006, "delete environment error", "删除环境信息失败"),
    DeleteEnvironmentRelatedTaskExists(AuroraData, Option<Vec<String>>), //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息"),
    QueryEnvironmentByNameError(AuroraData, Option<Vec<String>>), //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在"),
    QueryEnvironmentByCodeError(AuroraData, Option<Vec<String>>), //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在"),
    QueryEnvironmentError(AuroraData, Option<Vec<String>>), //(1200010, "login user query environment error", "分页查询环境列表错误"),
    VerifyEnvironmentError(AuroraData, Option<Vec<String>>), //(1200011, "verify environment error", "验证环境信息错误"),
    GetRuleFormCreateJsonError(AuroraData, Option<Vec<String>>), //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误"),
    QueryRuleListPagingError(AuroraData, Option<Vec<String>>), //(1200013, "query rule list paging error", "获取规则分页列表错误"),
    QueryRuleListError(AuroraData, Option<Vec<String>>),       //(1200014, "query rule list error", "获取规则列表错误"),
    QueryRuleInputEntryListError(AuroraData, Option<Vec<String>>), //(1200015, "query rule list error", "获取规则列表错误"),
    QueryExecuteResultListPagingError(AuroraData, Option<Vec<String>>), //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误"),
    GetDatasourceOptionsError(AuroraData, Option<Vec<String>>), //(1200017, "get datasource options error", "获取数据源Options错误"),
    GetDatasourceTablesError(AuroraData, Option<Vec<String>>), //(1200018, "get datasource tables error", "获取数据源表列表错误"),
    GetDatasourceTableColumnsError(AuroraData, Option<Vec<String>>), //(1200019, "get datasource table columns error", "获取数据源表列名错误"),
    TaskGroupNameExist(AuroraData, Option<Vec<String>>), //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用"),
    TaskGroupSizeError(AuroraData, Option<Vec<String>>), //(130002, "task group size error", "任务组大小应该为大于1的整数"),
    TaskGroupStatusError(AuroraData, Option<Vec<String>>), //(130003, "task group status error", "任务组已经被关闭"),
    TaskGroupFull(AuroraData, Option<Vec<String>>),      //(130004, "task group is full", "任务组已经满了"),
    TaskGroupUsedSizeError(AuroraData, Option<Vec<String>>), //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化"),
    TaskGroupQueueReleaseError(AuroraData, Option<Vec<String>>), //(130006, "failed to release task group queue", "任务组资源释放时出现了错误"),
    TaskGroupQueueAwakeError(AuroraData, Option<Vec<String>>), //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误"),
    CreateTaskGroupError(AuroraData, Option<Vec<String>>),     //(130008, "create task group error", "创建任务组错误"),
    UpdateTaskGroupError(AuroraData, Option<Vec<String>>), //(130009, "update task group list error", "更新任务组错误"),
    QueryTaskGroupListError(AuroraData, Option<Vec<String>>), //(130010, "query task group list error", "查询任务组列表错误"),
    CloseTaskGroupError(AuroraData, Option<Vec<String>>),     //(130011, "close task group error", "关闭任务组错误"),
    StartTaskGroupError(AuroraData, Option<Vec<String>>),     //(130012, "start task group error", "启动任务组错误"),
    QueryTaskGroupQueueListError(AuroraData, Option<Vec<String>>), //(130013, "query task group queue list error", "查询任务组队列列表错误"),
    TaskGroupCacheStartFailed(AuroraData, Option<Vec<String>>), //(130014, "cache start failed", "任务组相关的缓存启动失败"),
    EnvironmentWorkerGroupsIsInvalid(AuroraData, Option<Vec<String>>), //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误"),
    UpdateEnvironmentWorkerGroupRelationError(AuroraData, Option<Vec<String>>), //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中"),
    TaskGroupQueueAlreadyStart(AuroraData, Option<Vec<String>>), //(130017, "task group queue already start", "节点已经获取任务组资源"),
    TaskGroupStatusClosed(AuroraData, Option<Vec<String>>), //(130018, "The task group has been closed.", "任务组已经被关闭"),
    TaskGroupStatusOpened(AuroraData, Option<Vec<String>>), //(130019, "The task group has been opened.", "任务组已经被开启"),
    NotAllowToDisableOwnAccount(AuroraData, Option<Vec<String>>), //(130020, "Not allow to disable your own account", "不能停用自己的账号"),
    NotAllowToDeleteDefaultAlarmGroup(AuroraData, Option<Vec<String>>), //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组"),
    TimeZoneIllegal(AuroraData, Option<Vec<String>>), //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法"),

    QueryK8sNamespaceListPagingError(AuroraData, Option<Vec<String>>), //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误"),
    K8sNamespaceExist(AuroraData, Option<Vec<String>>), //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在"),
    CreateK8sNamespaceError(AuroraData, Option<Vec<String>>), //(1300003, "create k8s namespace error", "创建k8s命名空间错误"),
    UpdateK8sNamespaceError(AuroraData, Option<Vec<String>>), //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误"),
    K8sNamespaceNotExist(AuroraData, Option<Vec<String>>), //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在"),
    K8sClientOpsError(AuroraData, Option<Vec<String>>), //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]"),
    VerifyK8sNamespaceError(AuroraData, Option<Vec<String>>), //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误"),
    DeleteK8sNamespaceByIdError(AuroraData, Option<Vec<String>>), //(1300008, "delete k8s namespace by id error", "删除命名空间错误"),
    VerifyParameterNameFailed(AuroraData, Option<Vec<String>>), //(1300009, "The file name verify failed", "文件命名校验失败"),
    StoreOperateCreateError(AuroraData, Option<Vec<String>>), //(1300010, "create the resource failed", "存储操作失败"),
    GrantK8sNamespaceError(AuroraData, Option<Vec<String>>),  //(1300011, "grant namespace error", "授权资源错误"),
    QueryUnauthorizedNamespaceError(AuroraData, Option<Vec<String>>), //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误"),
    QueryAuthorizedNamespaceError(AuroraData, Option<Vec<String>>), //(1300013, "query authorized namespace error", "查询授权命名空间错误"),
    QueryCanUseK8sClusterError(AuroraData, Option<Vec<String>>), //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误"),
    ResourceFullNameTooLongError(AuroraData, Option<Vec<String>>), //(1300015, "resource's fullname is too long error", "资源文件名过长"),
    TenantFullNameTooLongError(AuroraData, Option<Vec<String>>), //(1300016, "tenant's fullname is too long error", "租户名过长");
}
impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        error!("{:<12} - model::Error {val:?}", "FROM_JSON");
        Self::InternalServerErrorArgs(AuroraData::String(val.to_string()), None)
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
            Error::SUCCESS(_, _) => tonic::Status::new(tonic::Code::Ok, "success"),
            _ => {
                let code = tonic::Code::Unknown;

                let info: AuroraErrorInfo = value.into();
                error!("{:<12} -  From<Error> for tonic::Status {info:#?}", "FROM_ERROR");
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
        Self::SUCCESS(AuroraData::Null, None)
    }
}

impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        if value.code() == tonic::Code::Ok {
            return Error::SUCCESS(AuroraData::Null, None);
        }
        if value.code() == tonic::Code::Internal {
            return Error::InternalServerErrorArgs(AuroraData::Null, None);
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
            let error_param = String::from("");

            let error_code = map.get("code").unwrap_or(&error_code);
            let cn_msg = map.get("cn_msg").unwrap_or(&cn_msg);
            let en_msg = map.get("en_msg").unwrap_or(&en_msg);
            let error_data = map.get("error_data").unwrap_or(&error_data);
            let error_param = map
                .get("error_param")
                .unwrap_or(&error_param)
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

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
                error_param: Some(error_param),
                // error_param: error_param.map(|s| s.to_string()),
            };
            let error: Error = error.into();
            error
        } else {
            Error::InternalServerErrorArgs(AuroraData::Null, None)
        }
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        error!("core::fmt::Display for Error -->{}", self);
        match self {
            Error::SUCCESS(data, _) => {
                let ss: AuroraErrorInfo = Error::SUCCESS(data.clone(), None).into();
                write!(f, "{}", ss)
            }
            Error::InternalServerErrorArgs(data, _param) => {
                let ss: AuroraErrorInfo = Error::InternalServerErrorArgs(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::RequestParamsNotValidError(data, _param) => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }

            Error::TaskTimeoutParamsError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskTimeoutParamsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNameExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNameNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserNameNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::HdfsOperationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::HdfsOperationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotFound(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotFound(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::OsTenantCodeExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::OsTenantCodeExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::AlertGroupNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::AlertGroupExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNamePasswdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserNamePasswdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::LoginSessionFailed(data, _param) => {
                let ss: AuroraErrorInfo = Error::LoginSessionFailed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceConnectFailed(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceConnectFailed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TenantNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::TenantNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectNotFound(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProjectNotFound(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectAlreadyExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProjectAlreadyExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotSubWorkflowInstance(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::TaskInstanceNotSubWorkflowInstance(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronNotExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronNotExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronOnlineForbidUpdate(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronOnlineForbidUpdate(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronCheckFailed(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronCheckFailed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MasterNotExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::MasterNotExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleStatusUnknown(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleStatusUnknown(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateAlertGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateAlertGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAllAlertgroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAllAlertgroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListPagingAlertGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ListPagingAlertGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateAlertGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateAlertGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteAlertGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupGrantUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::AlertGroupGrantUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateDatasourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateDatasourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateDatasourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateDatasourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDatasourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryDatasourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ConnectDatasourceFailure(data, _param) => {
                let ss: AuroraErrorInfo = Error::ConnectDatasourceFailure(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ConnectionTestFailure(data, _param) => {
                let ss: AuroraErrorInfo = Error::ConnectionTestFailure(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteDataSourceFailure(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteDataSourceFailure(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyDatasourceNameFailure(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyDatasourceNameFailure(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedDatasource(data, _param) => {
                let ss: AuroraErrorInfo = Error::UnauthorizedDatasource(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedDataSource(data, _param) => {
                let ss: AuroraErrorInfo = Error::AuthorizedDataSource(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::LoginSuccess(data, _param) => {
                let ss: AuroraErrorInfo = Error::LoginSuccess(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserLoginFailure(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserLoginFailure(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListWorkersError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ListWorkersError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListMastersError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ListMastersError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProjectDetailsByCodeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryProjectDetailsByCodeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::LoginUserQueryProjectListPagingError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::LoginUserQueryProjectListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUnauthorizedProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryUnauthorizedProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedProject(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedProject(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryQueueListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryQueueListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateResourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateResourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateResourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateResourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryResourcesListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryResourcesListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryResourcesListPaging(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryResourcesListPaging(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteResourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteResourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyResourceByNameAndTypeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyResourceByNameAndTypeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ViewResourceFileOnLineError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ViewResourceFileOnLineError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateResourceFileOnLineError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateResourceFileOnLineError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileIsEmpty(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceFileIsEmpty(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EditResourceFileOnLineError(data, _param) => {
                let ss: AuroraErrorInfo = Error::EditResourceFileOnLineError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DownloadResourceFileError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DownloadResourceFileError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ViewUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ViewUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUdfFunctionListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryUdfFunctionListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDatasourceByTypeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryDatasourceByTypeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyUdfFunctionNameError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyUdfFunctionNameError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedFileResourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::AuthorizedFileResourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizeResourceTree(data, _param) => {
                let ss: AuroraErrorInfo = Error::AuthorizeResourceTree(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UnauthorizedUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::AuthorizedUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateScheduleError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateScheduleError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateScheduleError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateScheduleError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PublishScheduleOnlineError(data, _param) => {
                let ss: AuroraErrorInfo = Error::PublishScheduleOnlineError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::OfflineScheduleError(data, _param) => {
                let ss: AuroraErrorInfo = Error::OfflineScheduleError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryScheduleListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryScheduleListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryScheduleListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryScheduleListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskRecordListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskRecordListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateTenantError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateTenantError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTenantListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTenantListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTenantListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTenantListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateTenantError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateTenantError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyOsTenantCodeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyOsTenantCodeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUserListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryUserListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteUserByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteUserByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GrantProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantResourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GrantResourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantUdfFunctionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GrantUdfFunctionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantDatasourceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GrantDatasourceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetUserInfoError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GetUserInfoError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyUsernameError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyUsernameError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UnauthorizedUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::AuthorizedUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskInstanceLogError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskInstanceLogError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DownloadTaskInstanceLogFileError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DownloadTaskInstanceLogFileError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyProcessDefinitionNameUniqueError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::VerifyProcessDefinitionNameUniqueError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ReleaseProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ReleaseProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDetailOfProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryDetailOfProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionList(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryProcessDefinitionList(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EncapsulationTreeviewStructureError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::EncapsulationTreeviewStructureError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetTasksListByProcessDefinitionIdError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::GetTasksListByProcessDefinitionIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceListPagingError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessInstanceListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskListByProcessInstanceIdError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryTaskListByProcessInstanceIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateProcessInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateProcessInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryProcessInstanceByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessInstanceByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteProcessInstanceByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QuerySubProcessInstanceDetailInfoByTaskIdError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QuerySubProcessInstanceDetailInfoByTaskIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(
                    data.clone(),
                    _param.clone(),
                )
                .into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceAllVariablesError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessInstanceAllVariablesError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EncapsulationProcessInstanceGanttStructureError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::EncapsulationProcessInstanceGanttStructureError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionListPagingError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessDefinitionListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SignOutError(data, _param) => {
                let ss: AuroraErrorInfo = Error::SignOutError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::OsTenantCodeHasAlreadyExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::OsTenantCodeHasAlreadyExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::IpIsEmpty(data, _param) => {
                let ss: AuroraErrorInfo = Error::IpIsEmpty(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronReleaseNeedNotChange(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronReleaseNeedNotChange(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateQueueError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateQueueError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueueNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueValueExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueueValueExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueNameExist(data, error_param) => {
                let ss: AuroraErrorInfo = Error::QueueNameExist(data.clone(), error_param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateQueueError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateQueueError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NeedNotUpdateQueue(data, _param) => {
                let ss: AuroraErrorInfo = Error::NeedNotUpdateQueue(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyQueueError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyQueueError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NameNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::NameNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NameExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::NameExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SaveError(data, _param) => {
                let ss: AuroraErrorInfo = Error::SaveError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProjectErrorDefinesNotNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteProjectErrorDefinesNotNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessInstanceByIdsError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::BatchDeleteProcessInstanceByIdsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PreviewScheduleError(data, _param) => {
                let ss: AuroraErrorInfo = Error::PreviewScheduleError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ParseToCronExpressionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ParseToCronExpressionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleStartTimeEndTimeSame(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleStartTimeEndTimeSame(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFailDefines(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFailDefines(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFailUsers(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFailUsers(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupByIdFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupByIdFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkerGroupFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryWorkerGroupFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserDisabled(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserDisabled(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CopyProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CopyProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MoveProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::MoveProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(data.clone(), _param.clone())
                        .into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(
                    data.clone(),
                    _param.clone(),
                )
                .into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionVersionsError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryProcessDefinitionVersionsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefinitionVersionError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteProcessDefinitionVersionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUserCreatedProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryUserCreatedProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionCodesIsEmpty(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionCodesIsEmpty(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchCopyProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::BatchCopyProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchMoveProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::BatchMoveProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkflowLineageError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryWorkflowLineageError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedAndUserCreatedProjectError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryAuthorizedAndUserCreatedProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefinitionByCodeFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefinitionByCodeFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CheckOsTenantCodeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CheckOsTenantCodeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ForceTaskSuccessError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ForceTaskSuccessError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceStateOperationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceStateOperationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceTypeNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceTypeNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionNameExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionNameExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceDbTypeIllegal(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceDbTypeIllegal(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourcePortIllegal(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourcePortIllegal(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceOtherParamsIllegal(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceOtherParamsIllegal(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceNameIllegal(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceNameIllegal(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DatasourceHostIllegal(data, _param) => {
                let ss: AuroraErrorInfo = Error::DatasourceHostIllegal(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateWorkerGroupForbiddenInDocker(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::CreateWorkerGroupForbiddenInDocker(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupForbiddenInDocker(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteWorkerGroupForbiddenInDocker(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::WorkerAddressInvalid(data, _param) => {
                let ss: AuroraErrorInfo = Error::WorkerAddressInvalid(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkerAddressListFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryWorkerAddressListFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TransformProjectOwnership(data, _param) => {
                let ss: AuroraErrorInfo = Error::TransformProjectOwnership(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAlertGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAlertGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CurrentLoginUserTenantNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::CurrentLoginUserTenantNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::RevokeProjectError(data, _param) => {
                let ss: AuroraErrorInfo = Error::RevokeProjectError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedUser(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedUser(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProjectNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceHostIsNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceHostIsNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryExecutingWorkflowError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryExecutingWorkflowError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfFunctionNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::UdfFunctionNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfFunctionExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::UdfFunctionExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceSuffixNotSupportView(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceSuffixNotSupportView(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceSizeExceedLimit(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceSizeExceedLimit(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceSuffixForbidChange(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceSuffixForbidChange(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfResourceSuffixNotJar(data, _param) => {
                let ss: AuroraErrorInfo = Error::UdfResourceSuffixNotJar(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::HdfsCopyFail(data, _param) => {
                let ss: AuroraErrorInfo = Error::HdfsCopyFail(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceFileExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceFileNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UdfResourceIsBound(data, _param) => {
                let ss: AuroraErrorInfo = Error::UdfResourceIsBound(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceIsUsed(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceIsUsed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ParentResourceNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ParentResourceNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceNotExistOrNoPermission(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceNotExistOrNoPermission(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceIsAuthorized(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceIsAuthorized(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNoOperationPerm(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserNoOperationPerm(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UserNoOperationProjectPerm(data, _param) => {
                let ss: AuroraErrorInfo = Error::UserNoOperationProjectPerm(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotRelease(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotRelease(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SubProcessDefineNotRelease(data, _param) => {
                let ss: AuroraErrorInfo = Error::SubProcessDefineNotRelease(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceAlreadyChanged(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceAlreadyChanged(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceStateOperationError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::ProcessInstanceStateOperationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SubProcessInstanceNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::SubProcessInstanceNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotAllowedEdit(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotAllowedEdit(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceExecutingCommand(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceExecutingCommand(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceNotSubProcessInstance(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::ProcessInstanceNotSubProcessInstance(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceStateCountError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskInstanceStateCountError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CountProcessInstanceStateError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CountProcessInstanceStateError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CountProcessDefinitionUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CountProcessDefinitionUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StartProcessInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::StartProcessInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchStartProcessInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::BatchStartProcessInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ExecuteProcessInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ExecuteProcessInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CheckProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CheckProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRecipientsAndCopyersByProcessDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryRecipientsAndCopyersByProcessDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DataIsNotValid(data, _param) => {
                let ss: AuroraErrorInfo = Error::DataIsNotValid(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DataIsNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::DataIsNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessNodeHasCycle(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessNodeHasCycle(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessNodeSParameterInvalid(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessNodeSParameterInvalid(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineStateOnline(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefineStateOnline(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefineByCodeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefineByCodeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronStateOnline(data, _param) => {
                let ss: AuroraErrorInfo = Error::ScheduleCronStateOnline(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteScheduleCronByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteScheduleCronByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessDefineError(data, _param) => {
                let ss: AuroraErrorInfo = Error::BatchDeleteProcessDefineError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessDefineByCodesError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::BatchDeleteProcessDefineByCodesError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefineByCodesError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefineByCodesError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TenantNotSuitable(data, _param) => {
                let ss: AuroraErrorInfo = Error::TenantNotSuitable(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ExportProcessDefineByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ExportProcessDefineByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::BatchExportProcessDefineByIdsError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::BatchExportProcessDefineByIdsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ImportProcessDefineError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ImportProcessDefineError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskDefineNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateProcessTaskRelationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateProcessTaskRelationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessTaskRelationNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessTaskRelationNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessTaskRelationExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessTaskRelationExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDagIsEmpty(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDagIsEmpty(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CheckProcessTaskRelationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CheckProcessTaskRelationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateTaskDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateTaskDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateTaskDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateTaskDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskDefinitionVersionsError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskDefinitionVersionsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::SwitchTaskDefinitionVersionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::SwitchTaskDefinitionVersionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskDefinitionVersionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTaskDefinitionVersionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskDefineByCodeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTaskDefineByCodeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDetailOfTaskDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryDetailOfTaskDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskDefinitionListPagingError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::QueryTaskDefinitionListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskDefinitionNameExisted(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskDefinitionNameExisted(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ReleaseTaskDefinitionError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ReleaseTaskDefinitionError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MoveProcessTaskRelationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::MoveProcessTaskRelationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskProcessRelationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteTaskProcessRelationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskProcessRelationError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskProcessRelationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskDefineStateOnline(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskDefineStateOnline(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskHasDownstream(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskHasDownstream(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskHasUpstream(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskHasUpstream(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::MainTableUsingVersion(data, _param) => {
                let ss: AuroraErrorInfo = Error::MainTableUsingVersion(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProjectProcessNotMatch(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProjectProcessNotMatch(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteEdgeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteEdgeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotSupportUpdateTaskDefinition(data, _param) => {
                let ss: AuroraErrorInfo = Error::NotSupportUpdateTaskDefinition(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotSupportCopyTaskType(data, _param) => {
                let ss: AuroraErrorInfo = Error::NotSupportCopyTaskType(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::HdfsNotStartup(data, _param) => {
                let ss: AuroraErrorInfo = Error::HdfsNotStartup(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StorageNotStartup(data, _param) => {
                let ss: AuroraErrorInfo = Error::StorageNotStartup(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::S3CannotRename(data, _param) => {
                let ss: AuroraErrorInfo = Error::S3CannotRename(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryDatabaseStateError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryDatabaseStateError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateAccessTokenError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateAccessTokenError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GenerateTokenError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GenerateTokenError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAccesstokenListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAccesstokenListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateAccessTokenError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateAccessTokenError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAccessTokenError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteAccessTokenError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::AccessTokenNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::AccessTokenNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAccesstokenByUserError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAccesstokenByUserError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CommandStateCountError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CommandStateCountError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NegativeSizeNumberError(data, _param) => {
                let ss: AuroraErrorInfo = Error::NegativeSizeNumberError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StartTimeBiggerThanEndTimeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::StartTimeBiggerThanEndTimeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueueCountError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueueCountError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::KerberosStartupState(data, _param) => {
                let ss: AuroraErrorInfo = Error::KerberosStartupState(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuditLogListPaging(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAuditLogListPaging(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PluginNotAUiComponent(data, _param) => {
                let ss: AuroraErrorInfo = Error::PluginNotAUiComponent(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginsResultIsNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryPluginsResultIsNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginsError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryPluginsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginDetailResultIsNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryPluginDetailResultIsNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateAlertPluginInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateAlertPluginInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertPluginInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteAlertPluginInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetAlertPluginInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GetAlertPluginInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateAlertPluginInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateAlertPluginInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAllAlertPluginInstanceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAllAlertPluginInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::PluginInstanceAlreadyExit(data, _param) => {
                let ss: AuroraErrorInfo = Error::PluginInstanceAlreadyExit(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ListPagingAlertPluginInstanceError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::ListPagingAlertPluginInstanceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionVersionIsUsed(data, _param) => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionVersionIsUsed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateEnvironmentError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateEnvironmentError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentNameExists(data, _param) => {
                let ss: AuroraErrorInfo = Error::EnvironmentNameExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentNameIsNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::EnvironmentNameIsNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentConfigIsNull(data, _param) => {
                let ss: AuroraErrorInfo = Error::EnvironmentConfigIsNull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateEnvironmentError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateEnvironmentError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteEnvironmentError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteEnvironmentError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteEnvironmentRelatedTaskExists(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::DeleteEnvironmentRelatedTaskExists(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentByNameError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentByNameError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentByCodeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentByCodeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyEnvironmentError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyEnvironmentError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetRuleFormCreateJsonError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GetRuleFormCreateJsonError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryRuleListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryRuleListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleInputEntryListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryRuleInputEntryListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryExecuteResultListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryExecuteResultListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceOptionsError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GetDatasourceOptionsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceTablesError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GetDatasourceTablesError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceTableColumnsError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GetDatasourceTableColumnsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupNameExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupNameExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupSizeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupSizeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupFull(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupFull(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupUsedSizeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupUsedSizeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueReleaseError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueReleaseError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueAwakeError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueAwakeError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateTaskGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateTaskGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateTaskGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateTaskGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskGroupListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskGroupListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CloseTaskGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CloseTaskGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StartTaskGroupError(data, _param) => {
                let ss: AuroraErrorInfo = Error::StartTaskGroupError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskGroupQueueListError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryTaskGroupQueueListError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupCacheStartFailed(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupCacheStartFailed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentWorkerGroupsIsInvalid(data, _param) => {
                let ss: AuroraErrorInfo = Error::EnvironmentWorkerGroupsIsInvalid(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateEnvironmentWorkerGroupRelationError(data, _param) => {
                let ss: AuroraErrorInfo =
                    Error::UpdateEnvironmentWorkerGroupRelationError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueAlreadyStart(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueAlreadyStart(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusClosed(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusClosed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusOpened(data, _param) => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusOpened(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotAllowToDisableOwnAccount(data, _param) => {
                let ss: AuroraErrorInfo = Error::NotAllowToDisableOwnAccount(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::NotAllowToDeleteDefaultAlarmGroup(data, _param) => {
                let ss: AuroraErrorInfo = Error::NotAllowToDeleteDefaultAlarmGroup(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TimeZoneIllegal(data, _param) => {
                let ss: AuroraErrorInfo = Error::TimeZoneIllegal(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryK8sNamespaceListPagingError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryK8sNamespaceListPagingError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::K8sNamespaceExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::K8sNamespaceExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::CreateK8sNamespaceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::CreateK8sNamespaceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::UpdateK8sNamespaceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::UpdateK8sNamespaceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::K8sNamespaceNotExist(data, _param) => {
                let ss: AuroraErrorInfo = Error::K8sNamespaceNotExist(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::K8sClientOpsError(data, _param) => {
                let ss: AuroraErrorInfo = Error::K8sClientOpsError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyK8sNamespaceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyK8sNamespaceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::DeleteK8sNamespaceByIdError(data, _param) => {
                let ss: AuroraErrorInfo = Error::DeleteK8sNamespaceByIdError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::VerifyParameterNameFailed(data, _param) => {
                let ss: AuroraErrorInfo = Error::VerifyParameterNameFailed(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::StoreOperateCreateError(data, _param) => {
                let ss: AuroraErrorInfo = Error::StoreOperateCreateError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::GrantK8sNamespaceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::GrantK8sNamespaceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryUnauthorizedNamespaceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryUnauthorizedNamespaceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedNamespaceError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedNamespaceError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::QueryCanUseK8sClusterError(data, _param) => {
                let ss: AuroraErrorInfo = Error::QueryCanUseK8sClusterError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::ResourceFullNameTooLongError(data, _param) => {
                let ss: AuroraErrorInfo = Error::ResourceFullNameTooLongError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
            Error::TenantFullNameTooLongError(data, _param) => {
                let ss: AuroraErrorInfo = Error::TenantFullNameTooLongError(data.clone(), _param.clone()).into();
                write!(f, "{}", ss)
            }
        }
    }
}

impl std::error::Error for Error {}
pub type AuroraData = serde_json::Value;
pub type ErrorParam = Option<Vec<String>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuroraErrorInfo {
    pub code: i32,
    // #[cfg(feature = "en_msg")]
    pub en_msg: String,
    // #[cfg(feature = "cn_msg")]
    pub cn_msg: String,
    pub error_data: AuroraData,
    pub error_param: ErrorParam,
}

impl Default for AuroraErrorInfo {
    fn default() -> Self {
        Self {
            code: 0,
            en_msg: "success".to_string(),
            cn_msg: "成功".to_string(),
            error_data: AuroraData::Null,
            error_param: None,
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
    pub fn new(code: i32, en_msg: String, cn_msg: String) -> Self {
        AuroraErrorInfo {
            code,
            en_msg,
            cn_msg,
            error_data: AuroraData::Null,
            error_param: None,
        }
    }
    pub fn new_with_data(&mut self, error_data: AuroraData) -> Self {
        AuroraErrorInfo {
            error_data,
            ..self.clone()
        }
    }
    pub fn parse(&mut self, error_param: ErrorParam) -> Self {
        let en_msg = AuroraErrorInfo::format_err_msg(&self.en_msg, error_param.clone());
        let cn_msg = AuroraErrorInfo::format_err_msg(&self.cn_msg, error_param.clone());
        AuroraErrorInfo {
            error_param,
            en_msg,
            cn_msg,
            ..self.clone()
        }
    }
    // #[warn(dead_code)]
    fn format_err_msg(text: &str, args: Option<Vec<String>>) -> String {
        if args.is_none() {
            return text.to_string();
        }
        let args = args.unwrap_or_default();
        let mut new_text = text.to_string();
        let re = regex::Regex::new(r"\{(\d+)").unwrap();
        for cap in re.captures_iter(text) {
            let index = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            if args.len() <= index {
                continue;
            }
            let ss = new_text.replace(&format!("{}{}{}", '{', index, '}'), &args[index]);
            new_text = ss.clone();
        }
        new_text
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
        let err_param = code_info[4]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .map(|x| serde_json::from_str::<ErrorParam>(x).unwrap_or_default());
        Ok(AuroraErrorInfo {
            code: *code,
            en_msg,
            cn_msg,
            error_data: err_data.unwrap_or_default(),
            error_param: err_param.unwrap_or_default(),
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
        let err_param = code_info[4]
            .split('|')
            .collect::<Vec<_>>()
            .get(1)
            .map(|x| serde_json::from_str::<ErrorParam>(x).unwrap_or_default());
        AuroraErrorInfo {
            code: *code,
            en_msg,
            cn_msg,
            error_data: err_data.unwrap_or_default(),
            error_param: err_param.unwrap_or_default(),
        }
    }
}

impl std::error::Error for AuroraErrorInfo {}
impl core::fmt::Display for AuroraErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        error!("AuroraErrorInfo: {:?}", self);
        let error_param = self.error_param.clone().unwrap_or_default();
        let error_param = error_param.join(",");
        write!(
            f,
            "code|{}~en_msg|{}~cn_msg|{}~error_data|{}~error_param|{:?}",
            self.code,
            self.en_msg,
            self.cn_msg,
            json!(self.error_data),
            error_param
        )
    }
}

#[test]
fn test_aururo_display() {
    let mut info = AuroraErrorInfo::new(10000, "en_msg".to_string(), "cn_msg".to_string());
    info.error_data = json!({"a":1});
    // info.error_param = Some(vec!["a".to_string(),"b".to_string()]);
    println!("{}", info);
}

impl From<AuroraErrorInfo> for Error {
    fn from(value: AuroraErrorInfo) -> Self {
        match (
            value.code,
            value.en_msg.as_str(),
            value.cn_msg.as_str(),
            value.error_data.clone(),
            value.error_param.clone(),
        ) {
            (0, ..) => Error::SUCCESS(value.error_data, None),
            (10000, ..) => Error::InternalServerErrorArgs(value.error_data, value.error_param),
            (10001, ..) => Error::RequestParamsNotValidError(value.error_data, value.error_param),
            (10002, ..) => Error::TaskTimeoutParamsError(value.error_data, value.error_param),
            (10003, ..) => Error::UserNameExist(value.error_data, value.error_param),
            (10004, ..) => Error::UserNameNull(value.error_data, value.error_param),
            (10006, ..) => Error::HdfsOperationError(value.error_data, value.error_param),
            (10018, ..) => Error::ProjectNotFound(value.error_data, value.error_param),
            (10019, ..) => Error::ProjectAlreadyExists(value.error_data, value.error_param),
            (10020, ..) => Error::TaskInstanceNotExists(value.error_data, value.error_param),
            (10030, ..) => Error::UpdateAlertGroupError(value.error_data, value.error_param), //(10030, "update alert group error", "更新告警组错误"),
            (10031, ..) => Error::DeleteAlertGroupError(value.error_data, value.error_param), //(10031, "delete alert group error", "删除告警组错误"),
            (10032, ..) => Error::AlertGroupGrantUserError(value.error_data, value.error_param), //(10032, "alert group grant user error", "告警组授权用户错误"),
            (10033, ..) => Error::CreateDatasourceError(value.error_data, value.error_param), //(10033, "create datasource error", "创建数据源错误"),
            (10034, ..) => Error::UpdateDatasourceError(value.error_data, value.error_param), //(10034, "update datasource error", "更新数据源错误"),
            (10035, ..) => Error::QueryDatasourceError(value.error_data, value.error_param), //(10035, "query datasource error", "查询数据源错误"),
            (10036, ..) => Error::ConnectDatasourceFailure(value.error_data, value.error_param), //(10036, "connect datasource failure", "建立数据源连接失败"),
            (10037, ..) => Error::ConnectionTestFailure(value.error_data, value.error_param), //(10037, "connection test failure", "测试数据源连接失败"),
            (10038, ..) => Error::DeleteDataSourceFailure(value.error_data, value.error_param), //(10038, "delete data source failure", "删除数据源失败"),
            (10039, ..) => Error::VerifyDatasourceNameFailure(value.error_data, value.error_param), //(10039, "verify datasource name failure", "验证数据源名称失败"),
            (10040, ..) => Error::UnauthorizedDatasource(value.error_data, value.error_param), //(10040, "unauthorized datasource", "未经授权的数据源"),
            (10041, ..) => Error::AuthorizedDataSource(value.error_data, value.error_param), //(10041, "authorized data source", "授权数据源失败"),
            (10042, ..) => Error::LoginSuccess(value.error_data, value.error_param), //(10042, "login success", "登录成功"),
            (10043, ..) => Error::UserLoginFailure(value.error_data, value.error_param), //(10043, "user login failure", "用户登录失败"),
            (10044, ..) => Error::ListWorkersError(value.error_data, value.error_param), //(10044, "list workers error", "查询worker列表错误"),
            (10045, ..) => Error::ListMastersError(value.error_data, value.error_param), //(10045, "list masters error", "查询master列表错误"),
            (10046, ..) => Error::UpdateProjectError(value.error_data, value.error_param), //(10046, "update project error", "更新项目信息错误"),
            (10047, ..) => Error::QueryProjectDetailsByCodeError(value.error_data, value.error_param), //(10047, "query project details by code error", "查询项目详细信息错误"),
            (10048, ..) => Error::CreateProjectError(value.error_data, value.error_param), //(10048, "create project error", "创建项目错误"),
            (10049, ..) => Error::LoginUserQueryProjectListPagingError(value.error_data, value.error_param), //(10049, "login user query project list paging error", "分页查询项目列表错误"),
            (10050, ..) => Error::DeleteProjectError(value.error_data, value.error_param), //(10050, "delete project error", "删除项目错误"),
            (10051, ..) => Error::QueryUnauthorizedProjectError(value.error_data, value.error_param), //(10051, "query unauthorized project error", "查询未授权项目错误"),
            (10052, ..) => Error::QueryAuthorizedProject(value.error_data, value.error_param), //(10052, "query authorized project", "查询授权项目错误"),
            (10053, ..) => Error::QueryQueueListError(value.error_data, value.error_param), //(10053, "query queue list error", "查询队列列表错误"),
            (10054, ..) => Error::CreateResourceError(value.error_data, value.error_param), //(10054, "create resource error", "创建资源错误"),
            (10055, ..) => Error::UpdateResourceError(value.error_data, value.error_param), //(10055, "update resource error", "更新资源错误"),
            (10056, ..) => Error::QueryResourcesListError(value.error_data, value.error_param), //(10056, "query resources list error", "查询资源列表错误"),
            (10057, "query resources list paging", "分页查询资源列表错误", ..) => {
                Error::QueryResourcesListPaging(value.error_data, value.error_param)
            }
            //(10057, "query resources list paging", "分页查询资源列表错误"),
            (10058, ..) => Error::DeleteResourceError(value.error_data, value.error_param), //(10058, "delete resource error", "删除资源错误"),
            (10059, ..) => Error::VerifyResourceByNameAndTypeError(value.error_data, value.error_param), //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
            (10060, ..) => Error::ViewResourceFileOnLineError(value.error_data, value.error_param), //(10060, "view resource file online error", "查看资源文件错误"),
            (10061, ..) => Error::CreateResourceFileOnLineError(value.error_data, value.error_param), //(10061, "create resource file online error", "创建资源文件错误"),
            (10062, ..) => Error::ResourceFileIsEmpty(value.error_data, value.error_param), //(10062, "resource file is empty", "资源文件内容不能为空"),
            (10063, ..) => Error::EditResourceFileOnLineError(value.error_data, value.error_param), //(10063, "edit resource file online error", "更新资源文件错误"),
            (10064, ..) => Error::DownloadResourceFileError(value.error_data, value.error_param), //(10064, "download resource file error", "下载资源文件错误"),
            (10065, ..) => Error::CreateUdfFunctionError(value.error_data, value.error_param), //(10065, "create udf function error", "创建UDF函数错误"),
            (10066, ..) => Error::ViewUdfFunctionError(value.error_data, value.error_param), //(10066, "view udf function error", "查询UDF函数错误"),
            (10067, ..) => Error::UpdateUdfFunctionError(value.error_data, value.error_param), //(10067, "update udf function error", "更新UDF函数错误"),
            (10068, ..) => Error::QueryUdfFunctionListPagingError(value.error_data, value.error_param), //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
            (10069, ..) => Error::QueryDatasourceByTypeError(value.error_data, value.error_param), //(10069, "query datasource by type error", "查询数据源信息错误"),
            (10070, ..) => Error::VerifyUdfFunctionNameError(value.error_data, value.error_param), //(10070, "verify udf function name error", "UDF函数名称验证错误"),
            (10071, ..) => Error::DeleteUdfFunctionError(value.error_data, value.error_param), //(10071, "delete udf function error", "删除UDF函数错误"),
            (10072, ..) => Error::AuthorizedFileResourceError(value.error_data, value.error_param), //(10072, "authorized file resource error", "授权资源文件错误"),
            (10073, ..) => Error::AuthorizeResourceTree(value.error_data, value.error_param), //(10073, "authorize resource tree display error", "授权资源目录树错误"),
            (10074, ..) => Error::UnauthorizedUdfFunctionError(value.error_data, value.error_param), //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
            (10075, ..) => Error::AuthorizedUdfFunctionError(value.error_data, value.error_param), //(10075, "authorized udf function error", "授权UDF函数错误"),
            (10076, ..) => Error::CreateScheduleError(value.error_data, value.error_param), //(10076, "create schedule error", "创建调度配置错误"),
            (10077, ..) => Error::UpdateScheduleError(value.error_data, value.error_param), //(10077, "update schedule error", "更新调度配置错误"),
            (10078, ..) => Error::PublishScheduleOnlineError(value.error_data, value.error_param), //(10078, "publish schedule online error", "上线调度配置错误"),
            (10079, ..) => Error::OfflineScheduleError(value.error_data, value.error_param), //(10079, "offline schedule error", "下线调度配置错误"),
            (10080, ..) => Error::QueryScheduleListPagingError(value.error_data, value.error_param), //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
            (10081, ..) => Error::QueryScheduleListError(value.error_data, value.error_param), //(10081, "query schedule list error", "查询调度配置列表错误"),
            (10082, ..) => Error::QueryTaskListPagingError(value.error_data, value.error_param), //(10082, "query task list paging error", "分页查询任务列表错误"),
            (10083, ..) => Error::QueryTaskRecordListPagingError(value.error_data, value.error_param), //(10083, "query task record list paging error", "分页查询任务记录错误"),
            (10084, ..) => Error::CreateTenantError(value.error_data, value.error_param), //(10084, "create tenant error", "创建租户错误"),
            (10085, ..) => Error::QueryTenantListPagingError(value.error_data, value.error_param), //(10085, "query tenant list paging error", "分页查询租户列表错误"),
            (10086, ..) => Error::QueryTenantListError(value.error_data, value.error_param), //(10086, "query tenant list error", "查询租户列表错误"),
            (10087, ..) => Error::UpdateTenantError(value.error_data, value.error_param), //(10087, "update tenant error", "更新租户错误"),
            (10088, ..) => Error::DeleteTenantByIdError(value.error_data, value.error_param), //(10088, "delete tenant by id error", "删除租户错误"),
            (10089, ..) => Error::VerifyOsTenantCodeError(value.error_data, value.error_param), //(10089, "verify os tenant code error", "操作系统租户验证错误"),
            (10090, ..) => Error::CreateUserError(value.error_data, value.error_param), //(10090, "create user error", "创建用户错误"),
            (10091, ..) => Error::QueryUserListPagingError(value.error_data, value.error_param), //(10091, "query user list paging error", "分页查询用户列表错误"),
            (10092, ..) => Error::UpdateUserError(value.error_data, value.error_param), //(10092, "update user error", "更新用户错误"),
            (10093, ..) => Error::DeleteUserByIdError(value.error_data, value.error_param), //(10093, "delete user by id error", "删除用户错误"),
            (10094, ..) => Error::GrantProjectError(value.error_data, value.error_param), //(10094, "grant project error", "授权项目错误"),
            (10095, ..) => Error::GrantResourceError(value.error_data, value.error_param), //(10095, "grant resource error", "授权资源错误"),
            (10096, ..) => Error::GrantUdfFunctionError(value.error_data, value.error_param), //(10096, "grant udf function error", "授权UDF函数错误"),
            (10097, ..) => Error::GrantDatasourceError(value.error_data, value.error_param), //(10097, "grant datasource error", "授权数据源错误"),
            (10098, ..) => Error::GetUserInfoError(value.error_data, value.error_param), //(10098, "get user info error", "获取用户信息错误"),
            (10099, ..) => Error::UserListError(value.error_data, value.error_param), //(10099, "user list error", "查询用户列表错误"),
            (10100, ..) => Error::VerifyUsernameError(value.error_data, value.error_param), //(10100, "verify username error", "用户名验证错误"),
            (10101, ..) => Error::UnauthorizedUserError(value.error_data, value.error_param), //(10101, "unauthorized user error", "查询未授权用户错误"),
            (10102, ..) => Error::AuthorizedUserError(value.error_data, value.error_param), //(10102, "authorized user error", "查询授权用户错误"),
            (10103, ..) => Error::QueryTaskInstanceLogError(value.error_data, value.error_param), //(10103, "view task instance log error", "查询任务实例日志错误"),
            (10104, ..) => Error::DownloadTaskInstanceLogFileError(value.error_data, value.error_param), //(10104, "download task instance log file error", "下载任务日志文件错误"),
            (10105, ..) => Error::CreateProcessDefinitionError(value.error_data, value.error_param), //(10105, "create process definition error", "创建工作流错误"),
            (10106, ..) => Error::VerifyProcessDefinitionNameUniqueError(value.error_data, value.error_param), //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
            (10107, ..) => Error::UpdateProcessDefinitionError(value.error_data, value.error_param), //(10107, "update process definition error", "更新工作流定义错误"),
            (10108, ..) => Error::ReleaseProcessDefinitionError(value.error_data, value.error_param), //(10108, "release process definition error", "上线工作流错误"),
            (10109, ..) => Error::QueryDetailOfProcessDefinitionError(value.error_data, value.error_param), //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
            (10110, ..) => Error::QueryProcessDefinitionList(value.error_data, value.error_param), //(10110, "query process definition list", "查询工作流列表错误"),
            (10111, ..) => Error::EncapsulationTreeviewStructureError(value.error_data, value.error_param), //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
            (10112, ..) => Error::GetTasksListByProcessDefinitionIdError(value.error_data, value.error_param), //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
            (10113, ..) => Error::QueryProcessInstanceListPagingError(value.error_data, value.error_param), //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
            (10114, ..) => Error::QueryTaskListByProcessInstanceIdError(value.error_data, value.error_param), //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
            (10115, ..) => Error::UpdateProcessInstanceError(value.error_data, value.error_param), //(10115, "update process instance error", "更新工作流实例错误"),
            (10116, ..) => Error::QueryProcessInstanceByIdError(value.error_data, value.error_param), //(10116, "query process instance by id error", "查询工作流实例错误"),
            (10117, "delete process instance by id error", "删除工作流实例错误", ..) => {
                Error::DeleteProcessInstanceByIdError(value.error_data, value.error_param)
            } //(10117, "delete process instance by id error", "删除工作流实例错误"),
            (10118, ..) => Error::QuerySubProcessInstanceDetailInfoByTaskIdError(value.error_data, value.error_param), //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
            (10119, ..) => Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(
                value.error_data,
                value.error_param,
            ), //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
            (10120, ..) => Error::QueryProcessInstanceAllVariablesError(value.error_data, value.error_param), //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
            (10121, ..) => Error::EncapsulationProcessInstanceGanttStructureError(value.error_data, value.error_param), //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
            (10122, ..) => Error::QueryProcessDefinitionListPagingError(value.error_data, value.error_param), //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
            (10123, ..) => Error::SignOutError(value.error_data, value.error_param), //(10123, "sign out error", "退出错误"),
            (10124, ..) => Error::OsTenantCodeHasAlreadyExists(value.error_data, value.error_param), //(10124, "os tenant code has already exists", "操作系统租户已存在"),
            (10125, ..) => Error::IpIsEmpty(value.error_data, value.error_param), //(10125, "ip is empty", "IP地址不能为空"),
            (10126, ..) => Error::ScheduleCronReleaseNeedNotChange(value.error_data, value.error_param), //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
            (10127, ..) => Error::CreateQueueError(value.error_data, value.error_param), //(10127, "create queue error", "创建队列错误"),
            (10128, ..) => Error::QueueNotExist(value.error_data, value.error_param), //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
            (10129, ..) => Error::QueueValueExist(value.error_data, value.error_param), //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
            (10130, ..) => Error::QueueNameExist(value.error_data, value.error_param), //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
            (10131, ..) => Error::UpdateQueueError(value.error_data, value.error_param), //(10131, "update queue error", "更新队列信息错误"),
            (10132, ..) => Error::NeedNotUpdateQueue(value.error_data, value.error_param), //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
            (10133, ..) => Error::VerifyQueueError(value.error_data, value.error_param), //(10133, "verify queue error", "验证队列信息错误"),
            (10134, ..) => Error::NameNull(value.error_data, value.error_param), //(10134, "name must be not null", "名称不能为空"),
            (10135, ..) => Error::NameExist(value.error_data, value.error_param), //(10135, "name {0} already exists", "名称[{0}]已存在"),
            (10136, ..) => Error::SaveError(value.error_data, value.error_param), //(10136, "save error", "保存错误"),
            (10117, "please delete the process definitions in project first!", "请先删除全部工作流定义", ..) => {
                Error::DeleteProjectErrorDefinesNotNull(value.error_data, value.error_param)
            } //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
            (10138, ..) => Error::BatchDeleteProcessInstanceByIdsError(value.error_data, value.error_param), //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
            (10139, ..) => Error::PreviewScheduleError(value.error_data, value.error_param), //(10139, "preview schedule error", "预览调度配置错误"),
            (10140, ..) => Error::ParseToCronExpressionError(value.error_data, value.error_param), //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
            (10141, ..) => Error::ScheduleStartTimeEndTimeSame(value.error_data, value.error_param), //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
            (10142, ..) => Error::DeleteTenantByIdFail(value.error_data, value.error_param), //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
            (10143, ..) => Error::DeleteTenantByIdFailDefines(value.error_data, value.error_param), //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
            (10144, ..) => Error::DeleteTenantByIdFailUsers(value.error_data, value.error_param), //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
            (10145, ..) => Error::DeleteWorkerGroupByIdFail(value.error_data, value.error_param), //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
            (10146, ..) => Error::QueryWorkerGroupFail(value.error_data, value.error_param), //(10146, "query worker group fail ", "查询worker分组失败"),
            (10147, ..) => Error::DeleteWorkerGroupFail(value.error_data, value.error_param), //(10147, "delete worker group fail ", "删除worker分组失败"),
            (10148, ..) => Error::UserDisabled(value.error_data, value.error_param), //(10148, "The current user is disabled", "当前用户已停用"),
            (10149, ..) => Error::CopyProcessDefinitionError(value.error_data, value.error_param), //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
            (10150, ..) => Error::MoveProcessDefinitionError(value.error_data, value.error_param), //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
            (10151, ..) => Error::SwitchProcessDefinitionVersionError(value.error_data, value.error_param), //(10151, "Switch process definition version error", "切换工作流版本出错"),
            (10152, ..) => {
                Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(value.error_data, value.error_param)
            } //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
            (10153, ..) => Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(
                value.error_data,
                value.error_param,
            ), //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
            (10154, ..) => Error::QueryProcessDefinitionVersionsError(value.error_data, value.error_param), //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
            (10156, ..) => Error::DeleteProcessDefinitionVersionError(value.error_data, value.error_param), //(10156, "delete process definition version error", "删除工作流历史版本出错"),

            (10157, ..) => Error::QueryUserCreatedProjectError(value.error_data, value.error_param), //(10157, "query user created project error error", "查询用户创建的项目错误"),
            (10158, ..) => Error::ProcessDefinitionCodesIsEmpty(value.error_data, value.error_param), //(10158, "process definition codes is empty", "工作流CODES不能为空"),
            (10159, ..) => Error::BatchCopyProcessDefinitionError(value.error_data, value.error_param), //(10159, "batch copy process definition error", "复制工作流错误"),
            (10160, ..) => Error::BatchMoveProcessDefinitionError(value.error_data, value.error_param), //(10160, "batch move process definition error", "移动工作流错误"),
            (10161, ..) => Error::QueryWorkflowLineageError(value.error_data, value.error_param), //(10161, "query workflow lineage error", "查询血缘失败"),
            (10162, ..) => Error::QueryAuthorizedAndUserCreatedProjectError(value.error_data, value.error_param), //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
            (10163, ..) => Error::DeleteProcessDefinitionByCodeFail(value.error_data, value.error_param), //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
            (10164, ..) => Error::CheckOsTenantCodeError(value.error_data, value.error_param), //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
            (10165, ..) => Error::ForceTaskSuccessError(value.error_data, value.error_param), //(10165, "force task success error", "强制成功任务实例错误"),
            (10166, ..) => Error::TaskInstanceStateOperationError(value.error_data, value.error_param), //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
            (10167, ..) => Error::DatasourceTypeNotExist(value.error_data, value.error_param), //(10167, "data source type not exist", "数据源类型不存在"),
            (10168, ..) => Error::ProcessDefinitionNameExist(value.error_data, value.error_param), //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
            (10169, ..) => Error::DatasourceDbTypeIllegal(value.error_data, value.error_param), //(10169, "datasource type illegal", "数据源类型参数不合法"),
            (10170, ..) => Error::DatasourcePortIllegal(value.error_data, value.error_param), //(10170, "datasource port illegal", "数据源端口参数不合法"),
            (10171, ..) => Error::DatasourceOtherParamsIllegal(value.error_data, value.error_param), //(10171, "datasource other params illegal", "数据源其他参数不合法"),
            (10172, ..) => Error::DatasourceNameIllegal(value.error_data, value.error_param), //(10172, "datasource name illegal", "数据源名称不合法"),
            (10173, ..) => Error::DatasourceHostIllegal(value.error_data, value.error_param), //(10173, "datasource host illegal", "数据源HOST不合法"),
            (10174, ..) => Error::DeleteWorkerGroupNotExist(value.error_data, value.error_param), //(10174, "delete worker group not exist ", "删除worker分组不存在"),
            (10175, ..) => Error::CreateWorkerGroupForbiddenInDocker(value.error_data, value.error_param), //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
            (10176, ..) => Error::DeleteWorkerGroupForbiddenInDocker(value.error_data, value.error_param), //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
            (10177, ..) => Error::WorkerAddressInvalid(value.error_data, value.error_param), //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
            (10178, ..) => Error::QueryWorkerAddressListFail(value.error_data, value.error_param), //(10178, "query worker address list fail ", "查询worker地址列表失败"),
            (10179, ..) => Error::TransformProjectOwnership(value.error_data, value.error_param), //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
            (10180, ..) => Error::QueryAlertGroupError(value.error_data, value.error_param), //(10180, "query alert group error", "查询告警组错误"),
            (10181, ..) => Error::CurrentLoginUserTenantNotExist(value.error_data, value.error_param), //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
            (10182, ..) => Error::RevokeProjectError(value.error_data, value.error_param), //(10182, "revoke project error", "撤销项目授权错误"),
            (10183, ..) => Error::QueryAuthorizedUser(value.error_data, value.error_param), //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
            (10184, ..) => Error::ProjectNotExist(value.error_data, value.error_param), //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
            (10185, ..) => Error::TaskInstanceHostIsNull(value.error_data, value.error_param), //(10191, "task instance host is null", "任务实例host为空"),
            (10186, ..) => Error::QueryExecutingWorkflowError(value.error_data, value.error_param), //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

            (20001, ..) => Error::UdfFunctionNotExist(value.error_data, value.error_param), //(20001, "UDF function not found", "UDF函数不存在"),
            (20002, ..) => Error::UdfFunctionExists(value.error_data, value.error_param), //(20002, "UDF function already exists", "UDF函数已存在"),
            (20004, ..) => Error::ResourceNotExist(value.error_data, value.error_param), //(20004, "resource not exist", "资源不存在"),
            (20005, ..) => Error::ResourceExist(value.error_data, value.error_param), //(20005, "resource already exists", "资源已存在"),
            (20006, ..) => Error::ResourceSuffixNotSupportView(value.error_data, value.error_param), //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
            (20007, ..) => Error::ResourceSizeExceedLimit(value.error_data, value.error_param), //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
            (20008, ..) => Error::ResourceSuffixForbidChange(value.error_data, value.error_param), //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
            (20009, ..) => Error::UdfResourceSuffixNotJar(value.error_data, value.error_param), //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
            (20010, ..) => Error::HdfsCopyFail(value.error_data, value.error_param), //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
            (20011, ..) => Error::ResourceFileExist(value.error_data, value.error_param), //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
            (20012, ..) => Error::ResourceFileNotExist(value.error_data, value.error_param), //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
            (20013, ..) => Error::UdfResourceIsBound(value.error_data, value.error_param), //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
            (20014, ..) => Error::ResourceIsUsed(value.error_data, value.error_param), //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
            (20015, ..) => Error::ParentResourceNotExist(value.error_data, value.error_param), //(20015, "parent resource not exist", "父资源文件不存在"),
            (20016, ..) => Error::ResourceNotExistOrNoPermission(value.error_data, value.error_param), //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
            (20017, ..) => Error::ResourceIsAuthorized(value.error_data, value.error_param), //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

            (30001, ..) => Error::UserNoOperationPerm(value.error_data, value.error_param), //(30001, "user has no operation privilege", "当前用户没有操作权限"),
            (30002, ..) => Error::UserNoOperationProjectPerm(value.error_data, value.error_param), //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

            (50001, ..) => Error::ProcessInstanceNotExist(value.error_data, value.error_param), //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
            (50002, ..) => Error::ProcessInstanceExist(value.error_data, value.error_param), //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
            (50003, ..) => Error::ProcessDefineNotExist(value.error_data, value.error_param), //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
            (
                50004,
                "process definition {0} process version {1} not online",
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态",
                ..,
            ) => Error::ProcessDefineNotRelease(value.error_data, value.error_param), //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
            (50004, "exist sub process definition not online", "存在子工作流定义不是上线状态", ..) => {
                Error::SubProcessDefineNotRelease(value.error_data, value.error_param)
            } //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
            (50005, ..) => Error::ProcessInstanceAlreadyChanged(value.error_data, value.error_param), //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
            (50006, ..) => Error::ProcessInstanceStateOperationError(value.error_data, value.error_param), //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
            (50007, ..) => Error::SubProcessInstanceNotExist(value.error_data, value.error_param), //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
            (50008, ..) => Error::ProcessDefineNotAllowedEdit(value.error_data, value.error_param), //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
            (50009, ..) => Error::ProcessInstanceExecutingCommand(value.error_data, value.error_param), //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
            (50010, ..) => Error::ProcessInstanceNotSubProcessInstance(value.error_data, value.error_param), //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
            (50011, ..) => Error::TaskInstanceStateCountError(value.error_data, value.error_param), //(50011, "task instance state count error", "查询各状态任务实例数错误"),
            (50012, ..) => Error::CountProcessInstanceStateError(value.error_data, value.error_param), //(50012, "count process instance state error", "查询各状态流程实例数错误"),
            (50013, ..) => Error::CountProcessDefinitionUserError(value.error_data, value.error_param), //(50013, "count process definition user error", "查询各用户流程定义数错误"),
            (50014, "start process instance error", "运行工作流实例错误", ..) => {
                Error::StartProcessInstanceError(value.error_data, value.error_param)
            } //(50014, "start process instance error", "运行工作流实例错误"),
            (50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}", ..) => {
                Error::BatchStartProcessInstanceError(value.error_data, value.error_param)
            } //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
            (50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误", ..) => {
                Error::ProcessInstanceError(value.error_data, value.error_param)
            } //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
            (50015, "execute process instance error", "操作工作流实例错误", ..) => {
                Error::ExecuteProcessInstanceError(value.error_data, value.error_param)
            } //(50015, "execute process instance error", "操作工作流实例错误")
            (50016, "check process definition error", "工作流定义错误", ..) => {
                Error::CheckProcessDefinitionError(value.error_data, value.error_param)
            } //(50016, "check process definition error", "工作流定义错误")
            (50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误", ..) => {
                Error::QueryRecipientsAndCopyersByProcessDefinitionError(value.error_data, value.error_param)
            } //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误")
            (50017, "data {0} not valid", "数据[{0}]无效", ..) => {
                Error::DataIsNotValid(value.error_data, value.error_param)
            } //(50017, "data {0} not valid", "数据[{0}]无效")
            (50018, "data {0} is null", "数据[{0}]不能为空", ..) => {
                Error::DataIsNull(value.error_data, value.error_param)
            } //(50018, "data {0} is null", "数据[{0}]不能为空")
            (50019, "process node has cycle", "流程节点间存在循环依赖", ..) => {
                Error::ProcessNodeHasCycle(value.error_data, value.error_param)
            } //(50019, "process node has cycle", "流程节点间存在循环依赖")
            (50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效", ..) => {
                Error::ProcessNodeSParameterInvalid(value.error_data, value.error_param)
            } //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效")
            (50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线", ..) => {
                Error::ProcessDefineStateOnline(value.error_data, value.error_param)
            } //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线")
            (50022, "delete process definition by code error", "删除工作流定义错误", ..) => {
                Error::DeleteProcessDefineByCodeError(value.error_data, value.error_param)
            } //(50022, "delete process definition by code error", "删除工作流定义错误")
            (50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线", ..) => {
                Error::ScheduleCronStateOnline(value.error_data, value.error_param)
            } //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线")
            (50024, "delete schedule by id error", "删除调度配置错误", ..) => {
                Error::DeleteScheduleCronByIdError(value.error_data, value.error_param)
            } //(50024, "delete schedule by id error", "删除调度配置错误")
            (50025, "batch delete process definition error", "批量删除工作流定义错误", ..) => {
                Error::BatchDeleteProcessDefineError(value.error_data, value.error_param)
            } //(50025, "batch delete process definition error", "批量删除工作流定义错误")
            (50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误", ..) => {
                Error::BatchDeleteProcessDefineByCodesError(value.error_data, value.error_param)
            } //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误")
            (50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误", ..) => {
                Error::DeleteProcessDefineByCodesError(value.error_data, value.error_param)
            } //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误")
            (
                50027,
                "there is not any tenant suitable, please choose a tenant available.",
                "没有合适的租户，请选择可用的租户",
                ..,
            ) => Error::TenantNotSuitable(value.error_data, value.error_param), //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户")
            (50028, "export process definition by id error", "导出工作流定义错误", ..) => {
                Error::ExportProcessDefineByIdError(value.error_data, value.error_param)
            } //(50028, "export process definition by id error", "导出工作流定义错误")
            (50028, "batch export process definition by ids error", "批量导出工作流定义错误", ..) => {
                Error::BatchExportProcessDefineByIdsError(value.error_data, value.error_param)
            } //(50028, "batch export process definition by ids error", "批量导出工作流定义错误")
            (50029, "import process definition error", "导入工作流定义错误", ..) => {
                Error::ImportProcessDefineError(value.error_data, value.error_param)
            } //(50029, "import process definition error", "导入工作流定义错误")
            (50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在", ..) => {
                Error::TaskDefineNotExist(value.error_data, value.error_param)
            } //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在")
            (50032, "create process task relation error", "创建工作流任务关系错误", ..) => {
                Error::CreateProcessTaskRelationError(value.error_data, value.error_param)
            } //(50032, "create process task relation error", "创建工作流任务关系错误")
            (50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在", ..) => {
                Error::ProcessTaskRelationNotExist(value.error_data, value.error_param)
            } //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在")
            (
                50034,
                "process task relation is already exist, processCode:[{0}]",
                "工作流任务关系已存在, processCode:[{0}]",
                ..,
            ) => Error::ProcessTaskRelationExist(value.error_data, value.error_param), //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]")
            (50035, "process dag is empty", "工作流dag是空", ..) => {
                Error::ProcessDagIsEmpty(value.error_data, value.error_param)
            } //(50035, "process dag is empty", "工作流dag是空")
            (50036, "check process task relation error", "工作流任务关系参数错误", ..) => {
                Error::CheckProcessTaskRelationError(value.error_data, value.error_param)
            } //(50036, "check process task relation error", "工作流任务关系参数错误")
            (50037, "create task definition error", "创建任务错误", ..) => {
                Error::CreateTaskDefinitionError(value.error_data, value.error_param)
            } //(50037, "create task definition error", "创建任务错误")
            (50038, "update task definition error", "更新任务定义错误", ..) => {
                Error::UpdateTaskDefinitionError(value.error_data, value.error_param)
            } //(50038, "update task definition error", "更新任务定义错误")
            (50039, "query task definition versions error", "查询任务历史版本信息出错", ..) => {
                Error::QueryTaskDefinitionVersionsError(value.error_data, value.error_param)
            } //(50039, "query task definition versions error", "查询任务历史版本信息出错")
            (50040, "Switch task definition version error", "切换任务版本出错", ..) => {
                Error::SwitchTaskDefinitionVersionError(value.error_data, value.error_param)
            } //(50040, "Switch task definition version error", "切换任务版本出错")
            (50041, "delete task definition version error", "删除任务历史版本出错", ..) => {
                Error::DeleteTaskDefinitionVersionError(value.error_data, value.error_param)
            } //(50041, "delete task definition version error", "删除任务历史版本出错")
            (50042, "delete task definition by code error", "删除任务定义错误", ..) => {
                Error::DeleteTaskDefineByCodeError(value.error_data, value.error_param)
            } //(50042, "delete task definition by code error", "删除任务定义错误")
            (50043, "query detail of task definition error", "查询任务详细信息错误", ..) => {
                Error::QueryDetailOfTaskDefinitionError(value.error_data, value.error_param)
            } //(50043, "query detail of task definition error", "查询任务详细信息错误")
            (50044, "query task definition list paging error", "分页查询任务定义列表错误", ..) => {
                Error::QueryTaskDefinitionListPagingError(value.error_data, value.error_param)
            } //(50044, "query task definition list paging error", "分页查询任务定义列表错误")
            (50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在", ..) => {
                Error::TaskDefinitionNameExisted(value.error_data, value.error_param)
            } //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在")
            (50046, "release task definition error", "上线任务错误", ..) => {
                Error::ReleaseTaskDefinitionError(value.error_data, value.error_param)
            } //(50046, "release task definition error", "上线任务错误")
            (50047, "move process task relation error", "移动任务到其他工作流错误", ..) => {
                Error::MoveProcessTaskRelationError(value.error_data, value.error_param)
            } //(50047, "move process task relation error", "移动任务到其他工作流错误")
            (50048, "delete process task relation error", "删除工作流任务关系错误", ..) => {
                Error::DeleteTaskProcessRelationError(value.error_data, value.error_param)
            } //(50048, "delete process task relation error", "删除工作流任务关系错误")
            (50049, "query process task relation error", "查询工作流任务关系错误", ..) => {
                Error::QueryTaskProcessRelationError(value.error_data, value.error_param)
            } //(50049, "query process task relation error", "查询工作流任务关系错误")
            (50050, "task definition [{0}] is already online", "任务定义[{0}]已上线", ..) => {
                Error::TaskDefineStateOnline(value.error_data, value.error_param)
            } //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线")
            (50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖", ..) => {
                Error::TaskHasDownstream(value.error_data, value.error_param)
            } //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖")
            (50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖", ..) => {
                Error::TaskHasUpstream(value.error_data, value.error_param)
            } //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖")
            (50053, "the version that the master table is using", "主表正在使用该版本", ..) => {
                Error::MainTableUsingVersion(value.error_data, value.error_param)
            } //(50053, "the version that the master table is using", "主表正在使用该版本")
            (50054, "the project and the process is not match", "项目和工作流不匹配", ..) => {
                Error::ProjectProcessNotMatch(value.error_data, value.error_param)
            } //(50054, "the project and the process is not match", "项目和工作流不匹配")
            (50055, "delete edge error", "删除工作流任务连接线错误", ..) => {
                Error::DeleteEdgeError(value.error_data, value.error_param)
            } //(50055, "delete edge error", "删除工作流任务连接线错误")
            (50056, "task state does not support modification", "当前任务不支持修改", ..) => {
                Error::NotSupportUpdateTaskDefinition(value.error_data, value.error_param)
            } //(50056, "task state does not support modification", "当前任务不支持修改")
            (50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]", ..) => {
                Error::NotSupportCopyTaskType(value.error_data, value.error_param)
            } //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]")
            (60001, "hdfs not startup", "hdfs未启用", ..) => {
                Error::HdfsNotStartup(value.error_data, value.error_param)
            } //(60001, "hdfs not startup", "hdfs未启用")
            (60002, "storage not startup", "存储未启用", ..) => {
                Error::StorageNotStartup(value.error_data, value.error_param)
            } //(60002, "storage not startup", "存储未启用")
            (60003, "directory cannot be renamed", "S3无法重命名文件夹", ..) => {
                Error::S3CannotRename(value.error_data, value.error_param)
            } //(60003, "directory cannot be renamed", "S3无法重命名文件夹")
            // for monitor
            (70001, "query database state error", "查询数据库状态错误", ..) => {
                Error::QueryDatabaseStateError(value.error_data, value.error_param)
            } //(70001, "query database state error", "查询数据库状态错误")

            (70010, ..) => Error::CreateAccessTokenError(value.error_data, value.error_param), //(70010, "create access token error", "创建访问token错误")
            (70011, ..) => Error::GenerateTokenError(value.error_data, value.error_param), //(70011, "generate token error", "生成token错误")
            (70012, ..) => Error::QueryAccesstokenListPagingError(value.error_data, value.error_param), //(70012, "query access token list paging error", "分页查询访问token列表错误")
            (70013, ..) => Error::UpdateAccessTokenError(value.error_data, value.error_param), //(70013, "update access token error", "更新访问token错误")
            (70014, ..) => Error::DeleteAccessTokenError(value.error_data, value.error_param), //(70014, "delete access token error", "删除访问token错误")
            (70015, ..) => Error::AccessTokenNotExist(value.error_data, value.error_param), //(70015, "access token not exist", "访问token不存在")
            (70016, ..) => Error::QueryAccesstokenByUserError(value.error_data, value.error_param), //(70016, "query access token by user error", "查询访问指定用户的token错误")

            (80001, ..) => Error::CommandStateCountError(value.error_data, value.error_param), //(80001, "task instance state count error", "查询各状态任务实例数错误")
            (80002, ..) => Error::NegativeSizeNumberError(value.error_data, value.error_param), //(80002, "query size number error", "查询size错误")
            (80003, ..) => Error::StartTimeBiggerThanEndTimeError(value.error_data, value.error_param), //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误")
            (90001, ..) => Error::QueueCountError(value.error_data, value.error_param), //(90001, "queue count error", "查询队列数据错误")

            (100001, ..) => Error::KerberosStartupState(value.error_data, value.error_param), //(100001, "get kerberos startup state error", "获取kerberos启动状态错误")

            // audit log
            (10057, "query audit log list paging", "分页查询日志列表错误", ..) => {
                Error::QueryAuditLogListPaging(value.error_data, value.error_param)
            } //(10057, "query audit log list paging", "分页查询日志列表错误")

            //plugin
            (110001, ..) => Error::PluginNotAUiComponent(value.error_data, value.error_param), //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件")
            (110002, ..) => Error::QueryPluginsResultIsNull(value.error_data, value.error_param), //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功")
            (110003, ..) => Error::QueryPluginsError(value.error_data, value.error_param), //(110003, "query plugins error", "查询插件错误")
            (110004, ..) => Error::QueryPluginDetailResultIsNull(value.error_data, value.error_param), //(110004, "query plugin detail result is null", "查询插件详情结果为空")

            (110005, ..) => Error::UpdateAlertPluginInstanceError(value.error_data, value.error_param), //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误")
            (110006, ..) => Error::DeleteAlertPluginInstanceError(value.error_data, value.error_param), //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误")
            (110007, ..) => Error::GetAlertPluginInstanceError(value.error_data, value.error_param), //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误")
            (110008, ..) => Error::CreateAlertPluginInstanceError(value.error_data, value.error_param), //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误")
            (110009, ..) => Error::QueryAllAlertPluginInstanceError(value.error_data, value.error_param), //(110009, "query all alert plugin instance error", "查询所有告警实例失败")
            (110010, ..) => Error::PluginInstanceAlreadyExit(value.error_data, value.error_param), //(110010, "plugin instance already exit", "该告警插件实例已存在")
            (110011, ..) => Error::ListPagingAlertPluginInstanceError(value.error_data, value.error_param), //(110011, "query plugin instance page error", "分页查询告警实例失败")
            (110012, ..) => {
                Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(value.error_data, value.error_param)
            } //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组")
            (110013, ..) => Error::ProcessDefinitionVersionIsUsed(value.error_data, value.error_param), //(110013, "this process definition version is used", "此工作流定义版本被使用")

            (120001, ..) => Error::CreateEnvironmentError(value.error_data, value.error_param), //(120001, "create environment error", "创建环境失败")
            (120002, ..) => Error::EnvironmentNameExists(value.error_data, value.error_param), //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在")
            (120003, ..) => Error::EnvironmentNameIsNull(value.error_data, value.error_param), //(120003, "this environment name shouldn't be empty.", "环境名称不能为空")
            (120004, ..) => Error::EnvironmentConfigIsNull(value.error_data, value.error_param), //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空")
            (120005, ..) => Error::UpdateEnvironmentError(value.error_data, value.error_param), //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败")
            (120006, ..) => Error::DeleteEnvironmentError(value.error_data, value.error_param), //(120006, "delete environment error", "删除环境信息失败")
            (120007, ..) => Error::DeleteEnvironmentRelatedTaskExists(value.error_data, value.error_param), //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息")
            (1200008, ..) => Error::QueryEnvironmentByNameError(value.error_data, value.error_param), //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在")
            (1200009, ..) => Error::QueryEnvironmentByCodeError(value.error_data, value.error_param), //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在")
            (1200010, ..) => Error::QueryEnvironmentError(value.error_data, value.error_param), //(1200010, "login user query environment error", "分页查询环境列表错误")
            (1200011, ..) => Error::VerifyEnvironmentError(value.error_data, value.error_param), //(1200011, "verify environment error", "验证环境信息错误")
            (1200012, ..) => Error::GetRuleFormCreateJsonError(value.error_data, value.error_param), //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误")
            (1200013, ..) => Error::QueryRuleListPagingError(value.error_data, value.error_param), //(1200013, "query rule list paging error", "获取规则分页列表错误")
            (1200014, ..) => Error::QueryRuleListError(value.error_data, value.error_param), //(1200014, "query rule list error", "获取规则列表错误")
            (1200015, ..) => Error::QueryRuleInputEntryListError(value.error_data, value.error_param), //(1200015, "query rule list error", "获取规则列表错误")
            (1200016, ..) => Error::QueryExecuteResultListPagingError(value.error_data, value.error_param), //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误")
            (1200017, ..) => Error::GetDatasourceOptionsError(value.error_data, value.error_param), //(1200017, "get datasource options error", "获取数据源Options错误")
            (1200018, ..) => Error::GetDatasourceTablesError(value.error_data, value.error_param), //(1200018, "get datasource tables error", "获取数据源表列表错误")
            (1200019, ..) => Error::GetDatasourceTableColumnsError(value.error_data, value.error_param), //(1200019, "get datasource table columns error", "获取数据源表列名错误")
            (130001, ..) => Error::TaskGroupNameExist(value.error_data, value.error_param), //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用")
            (130002, ..) => Error::TaskGroupSizeError(value.error_data, value.error_param), //(130002, "task group size error", "任务组大小应该为大于1的整数")
            (130003, ..) => Error::TaskGroupStatusError(value.error_data, value.error_param), //(130003, "task group status error", "任务组已经被关闭")
            (130004, ..) => Error::TaskGroupFull(value.error_data, value.error_param), //(130004, "task group is full", "任务组已经满了")
            (130005, ..) => Error::TaskGroupUsedSizeError(value.error_data, value.error_param), //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化")
            (130006, ..) => Error::TaskGroupQueueReleaseError(value.error_data, value.error_param), //(130006, "failed to release task group queue", "任务组资源释放时出现了错误")
            (130007, ..) => Error::TaskGroupQueueAwakeError(value.error_data, value.error_param), //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误")
            (130008, ..) => Error::CreateTaskGroupError(value.error_data, value.error_param), //(130008, "create task group error", "创建任务组错误")
            (130009, ..) => Error::UpdateTaskGroupError(value.error_data, value.error_param), //(130009, "update task group list error", "更新任务组错误")
            (130010, ..) => Error::QueryTaskGroupListError(value.error_data, value.error_param), //(130010, "query task group list error", "查询任务组列表错误")
            (130011, ..) => Error::CloseTaskGroupError(value.error_data, value.error_param), //(130011, "close task group error", "关闭任务组错误")
            (130012, ..) => Error::StartTaskGroupError(value.error_data, value.error_param), //(130012, "start task group error", "启动任务组错误")
            (130013, ..) => Error::QueryTaskGroupQueueListError(value.error_data, value.error_param), //(130013, "query task group queue list error", "查询任务组队列列表错误")
            (130014, ..) => Error::TaskGroupCacheStartFailed(value.error_data, value.error_param), //(130014, "cache start failed", "任务组相关的缓存启动失败")
            (130015, ..) => Error::EnvironmentWorkerGroupsIsInvalid(value.error_data, value.error_param), //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误")
            (130016, ..) => Error::UpdateEnvironmentWorkerGroupRelationError(value.error_data, value.error_param), //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中")
            (130017, ..) => Error::TaskGroupQueueAlreadyStart(value.error_data, value.error_param), //(130017, "task group queue already start", "节点已经获取任务组资源")
            (130018, ..) => Error::TaskGroupStatusClosed(value.error_data, value.error_param), //(130018, "The task group has been closed.", "任务组已经被关闭")
            (130019, ..) => Error::TaskGroupStatusOpened(value.error_data, value.error_param), //(130019, "The task group has been opened.", "任务组已经被开启")
            (130020, ..) => Error::NotAllowToDisableOwnAccount(value.error_data, value.error_param), //(130020, "Not allow to disable your own account", "不能停用自己的账号")
            (130030, ..) => Error::NotAllowToDeleteDefaultAlarmGroup(value.error_data, value.error_param), //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组")
            (130031, ..) => Error::TimeZoneIllegal(value.error_data, value.error_param), //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法")

            (1300001, ..) => Error::QueryK8sNamespaceListPagingError(value.error_data, value.error_param), //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误")
            (1300002, ..) => Error::K8sNamespaceExist(value.error_data, value.error_param), //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在")
            (1300003, ..) => Error::CreateK8sNamespaceError(value.error_data, value.error_param), //(1300003, "create k8s namespace error", "创建k8s命名空间错误")
            (1300004, ..) => Error::UpdateK8sNamespaceError(value.error_data, value.error_param), //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误")
            (1300005, ..) => Error::K8sNamespaceNotExist(value.error_data, value.error_param), //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在")
            (1300006, ..) => Error::K8sClientOpsError(value.error_data, value.error_param), //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]")
            (1300007, ..) => Error::VerifyK8sNamespaceError(value.error_data, value.error_param), //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误")
            (1300008, ..) => Error::DeleteK8sNamespaceByIdError(value.error_data, value.error_param), //(1300008, "delete k8s namespace by id error", "删除命名空间错误")
            (1300009, ..) => Error::VerifyParameterNameFailed(value.error_data, value.error_param), //(1300009, "The file name verify failed", "文件命名校验失败")
            (1300010, ..) => Error::StoreOperateCreateError(value.error_data, value.error_param), //(1300010, "create the resource failed", "存储操作失败")
            (1300011, ..) => Error::GrantK8sNamespaceError(value.error_data, value.error_param), //(1300011, "grant namespace error", "授权资源错误")
            (1300012, ..) => Error::QueryUnauthorizedNamespaceError(value.error_data, value.error_param), //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误")
            (1300013, ..) => Error::QueryAuthorizedNamespaceError(value.error_data, value.error_param), //(1300013, "query authorized namespace error", "查询授权命名空间错误")
            (1300014, ..) => Error::QueryCanUseK8sClusterError(value.error_data, value.error_param), //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误")
            (1300015, ..) => Error::ResourceFullNameTooLongError(value.error_data, value.error_param), //(1300015, "resource's fullname is too long error", "资源文件名过长")
            (1300016, ..) => Error::TenantFullNameTooLongError(value.error_data, value.error_param), //(1300016, "tenant's fullname is too long error", "租户名过长");
            (..) => Error::InternalServerErrorArgs(value.error_data, value.error_param),
        }
    }
}

impl From<Error> for AuroraErrorInfo {
    fn from(status: Error) -> Self {
        match status {
            Error::SUCCESS(_, _) => AuroraErrorInfo::new(0, "success".to_string(), "成功".to_string()),
            Error::InternalServerErrorArgs(data, _param) => AuroraErrorInfo::new(
                10000,
                "internal server error please check the log".to_string(),
                "内部服务错误，请查看日志".to_string(),
            )
            .new_with_data(data),
            Error::RequestParamsNotValidError(data, _param) => AuroraErrorInfo::new(
                10001,
                "request parameter {0} is not valid".to_string(),
                "请求参数[{0}]无效".to_string(),
            )
            .new_with_data(data),
            Error::TaskTimeoutParamsError(data, _param) => AuroraErrorInfo::new(
                10002,
                "task timeout parameter is not valid".to_string(),
                "任务超时参数无效".to_string(),
            )
            .new_with_data(data),
            Error::UserNameExist(data, _param) => AuroraErrorInfo::new(
                10003,
                "user name already exists".to_string(),
                "用户名已存在".to_string(),
            )
            .new_with_data(data),
            Error::UserNameNull(data, _param) => {
                AuroraErrorInfo::new(10004, "user name is null".to_string(), "用户名不能为空".to_string())
                    .new_with_data(data)
            }
            Error::HdfsOperationError(data, _param) => {
                AuroraErrorInfo::new(10006, "hdfs operation error".to_string(), "hdfs操作错误".to_string())
                    .new_with_data(data)
            }
            Error::TaskInstanceNotFound(data, _param) => AuroraErrorInfo::new(
                10008,
                "task instance not found".to_string(),
                "任务实例不存在".to_string(),
            )
            .new_with_data(data),
            Error::OsTenantCodeExist(data, _param) => AuroraErrorInfo::new(
                10009,
                "os tenant code {0} already exists".to_string(),
                "操作系统租户[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::UserNotExist(data, _param) => {
                AuroraErrorInfo::new(10010, "user {0} not exists".to_string(), "用户[{0}]不存在".to_string())
                    .new_with_data(data)
            }
            Error::AlertGroupNotExist(data, _param) => {
                AuroraErrorInfo::new(10011, "alarm group not found".to_string(), "告警组不存在".to_string())
                    .new_with_data(data)
            }
            Error::AlertGroupExist(data, _param) => AuroraErrorInfo::new(
                10012,
                "alarm group already exists".to_string(),
                "告警组名称已存在".to_string(),
            )
            .new_with_data(data),
            Error::UserNamePasswdError(data, _param) => AuroraErrorInfo::new(
                10013,
                "user name or password error".to_string(),
                "用户名或密码错误".to_string(),
            )
            .new_with_data(data),
            Error::LoginSessionFailed(data, _param) => AuroraErrorInfo::new(
                10014,
                "create session failed!".to_string(),
                "创建session失败".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceExist(data, _param) => AuroraErrorInfo::new(
                10015,
                "data source name already exists".to_string(),
                "数据源名称已存在".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceConnectFailed(data, _param) => AuroraErrorInfo::new(
                10016,
                "data source connection failed".to_string(),
                "建立数据源连接失败".to_string(),
            )
            .new_with_data(data),
            Error::TenantNotExist(data, _param) => {
                AuroraErrorInfo::new(10017, "tenant not exists".to_string(), "租户不存在".to_string())
                    .new_with_data(data)
            }
            Error::ProjectNotFound(data, _param) => AuroraErrorInfo::new(
                10018,
                "project {0} not found ".to_string(),
                "项目[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::ProjectAlreadyExists(data, _param) => AuroraErrorInfo::new(
                10019,
                "project {0} already exists".to_string(),
                "项目名称[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::TaskInstanceNotExists(data, _param) => AuroraErrorInfo::new(
                10020,
                "task instance {0} does not exist".to_string(),
                "任务实例[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::TaskInstanceNotSubWorkflowInstance(data, _param) => AuroraErrorInfo::new(
                10021,
                "task instance {0} is not sub process instance".to_string(),
                "任务实例[{0}]不是子流程实例".to_string(),
            )
            .new_with_data(data),
            Error::ScheduleCronNotExists(data, _param) => AuroraErrorInfo::new(
                10022,
                "scheduler crontab {0} does not exist".to_string(),
                "调度配置定时表达式[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::ScheduleCronOnlineForbidUpdate(data, _param) => AuroraErrorInfo::new(
                10023,
                "online status does not allow update operations".to_string(),
                "调度配置上线状态不允许修改".to_string(),
            )
            .new_with_data(data),
            Error::ScheduleCronCheckFailed(data, _param) => AuroraErrorInfo::new(
                10024,
                "scheduler crontab expression validation failure: {0}".to_string(),
                "调度配置定时表达式验证失败: {0}".to_string(),
            )
            .new_with_data(data),
            Error::MasterNotExists(data, _param) => AuroraErrorInfo::new(
                10025,
                "master does not exist".to_string(),
                "无可用master节点".to_string(),
            )
            .new_with_data(data),
            Error::ScheduleStatusUnknown(data, _param) => {
                AuroraErrorInfo::new(10026, "unknown status: {0}".to_string(), "未知状态: {0}".to_string())
                    .new_with_data(data)
            }
            Error::CreateAlertGroupError(data, _param) => AuroraErrorInfo::new(
                10027,
                "create alert group error".to_string(),
                "创建告警组错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryAllAlertgroupError(data, _param) => AuroraErrorInfo::new(
                10028,
                "query all alertgroup error".to_string(),
                "查询告警组错误".to_string(),
            )
            .new_with_data(data),
            Error::ListPagingAlertGroupError(data, _param) => AuroraErrorInfo::new(
                10029,
                "list paging alert group error".to_string(),
                "分页查询告警组错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateAlertGroupError(data, _param) => AuroraErrorInfo::new(
                10030,
                "update alert group error".to_string(),
                "更新告警组错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteAlertGroupError(data, _param) => AuroraErrorInfo::new(
                10031,
                "delete alert group error".to_string(),
                "删除告警组错误".to_string(),
            )
            .new_with_data(data),
            Error::AlertGroupGrantUserError(data, _param) => AuroraErrorInfo::new(
                10032,
                "alert group grant user error".to_string(),
                "告警组授权用户错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateDatasourceError(data, _param) => AuroraErrorInfo::new(
                10033,
                "create datasource error".to_string(),
                "创建数据源错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateDatasourceError(data, _param) => AuroraErrorInfo::new(
                10034,
                "update datasource error".to_string(),
                "更新数据源错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryDatasourceError(data, _param) => AuroraErrorInfo::new(
                10035,
                "query datasource error".to_string(),
                "查询数据源错误".to_string(),
            )
            .new_with_data(data),
            Error::ConnectDatasourceFailure(data, _param) => AuroraErrorInfo::new(
                10036,
                "connect datasource failure".to_string(),
                "建立数据源连接失败".to_string(),
            )
            .new_with_data(data),
            Error::ConnectionTestFailure(data, _param) => AuroraErrorInfo::new(
                10037,
                "connection test failure".to_string(),
                "测试数据源连接失败".to_string(),
            )
            .new_with_data(data),
            Error::DeleteDataSourceFailure(data, _param) => AuroraErrorInfo::new(
                10038,
                "delete data source failure".to_string(),
                "删除数据源失败".to_string(),
            )
            .new_with_data(data),
            Error::VerifyDatasourceNameFailure(data, _param) => AuroraErrorInfo::new(
                10039,
                "verify datasource name failure".to_string(),
                "验证数据源名称失败".to_string(),
            )
            .new_with_data(data),
            Error::UnauthorizedDatasource(data, _param) => AuroraErrorInfo::new(
                10040,
                "unauthorized datasource".to_string(),
                "未经授权的数据源".to_string(),
            )
            .new_with_data(data),
            Error::AuthorizedDataSource(data, _param) => AuroraErrorInfo::new(
                10041,
                "authorized data source".to_string(),
                "授权数据源失败".to_string(),
            )
            .new_with_data(data),
            Error::LoginSuccess(data, _param) => {
                AuroraErrorInfo::new(10042, "login success".to_string(), "登录成功".to_string()).new_with_data(data)
            }
            Error::UserLoginFailure(data, _param) => {
                AuroraErrorInfo::new(10043, "user login failure".to_string(), "用户登录失败".to_string())
                    .new_with_data(data)
            }
            Error::ListWorkersError(data, _param) => AuroraErrorInfo::new(
                10044,
                "list workers error".to_string(),
                "查询worker列表错误".to_string(),
            )
            .new_with_data(data),
            Error::ListMastersError(data, _param) => AuroraErrorInfo::new(
                10045,
                "list masters error".to_string(),
                "查询master列表错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateProjectError(data, _param) => AuroraErrorInfo::new(
                10046,
                "update project error".to_string(),
                "更新项目信息错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryProjectDetailsByCodeError(data, _param) => AuroraErrorInfo::new(
                10047,
                "query project details by code error".to_string(),
                "查询项目详细信息错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateProjectError(data, _param) => {
                AuroraErrorInfo::new(10048, "create project error".to_string(), "创建项目错误".to_string())
                    .new_with_data(data)
            }
            Error::LoginUserQueryProjectListPagingError(data, _param) => AuroraErrorInfo::new(
                10049,
                "login user query project list paging error".to_string(),
                "分页查询项目列表错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteProjectError(data, _param) => {
                AuroraErrorInfo::new(10050, "delete project error".to_string(), "删除项目错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryUnauthorizedProjectError(data, _param) => AuroraErrorInfo::new(
                10051,
                "query unauthorized project error".to_string(),
                "查询未授权项目错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryAuthorizedProject(data, _param) => AuroraErrorInfo::new(
                10052,
                "query authorized project".to_string(),
                "查询授权项目错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryQueueListError(data, _param) => AuroraErrorInfo::new(
                10053,
                "query queue list error".to_string(),
                "查询队列列表错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateResourceError(data, _param) => {
                AuroraErrorInfo::new(10054, "create resource error".to_string(), "创建资源错误".to_string())
                    .new_with_data(data)
            }
            Error::UpdateResourceError(data, _param) => {
                AuroraErrorInfo::new(10055, "update resource error".to_string(), "更新资源错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryResourcesListError(data, _param) => AuroraErrorInfo::new(
                10056,
                "query resources list error".to_string(),
                "查询资源列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryResourcesListPaging(data, _param) => AuroraErrorInfo::new(
                10057,
                "query resources list paging".to_string(),
                "分页查询资源列表错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteResourceError(data, _param) => {
                AuroraErrorInfo::new(10058, "delete resource error".to_string(), "删除资源错误".to_string())
                    .new_with_data(data)
            }
            Error::VerifyResourceByNameAndTypeError(data, _param) => AuroraErrorInfo::new(
                10059,
                "verify resource by name and type error".to_string(),
                "资源名称或类型验证错误".to_string(),
            )
            .new_with_data(data),
            Error::ViewResourceFileOnLineError(data, _param) => AuroraErrorInfo::new(
                10060,
                "view resource file online error".to_string(),
                "查看资源文件错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateResourceFileOnLineError(data, _param) => AuroraErrorInfo::new(
                10061,
                "create resource file online error".to_string(),
                "创建资源文件错误".to_string(),
            )
            .new_with_data(data),
            Error::ResourceFileIsEmpty(data, _param) => AuroraErrorInfo::new(
                10062,
                "resource file is empty".to_string(),
                "资源文件内容不能为空".to_string(),
            )
            .new_with_data(data),
            Error::EditResourceFileOnLineError(data, _param) => AuroraErrorInfo::new(
                10063,
                "edit resource file online error".to_string(),
                "更新资源文件错误".to_string(),
            )
            .new_with_data(data),
            Error::DownloadResourceFileError(data, _param) => AuroraErrorInfo::new(
                10064,
                "download resource file error".to_string(),
                "下载资源文件错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10065,
                "create udf function error".to_string(),
                "创建UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::ViewUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10066,
                "view udf function error".to_string(),
                "查询UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10067,
                "update udf function error".to_string(),
                "更新UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryUdfFunctionListPagingError(data, _param) => AuroraErrorInfo::new(
                10068,
                "query udf function list paging error".to_string(),
                "分页查询UDF函数列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryDatasourceByTypeError(data, _param) => AuroraErrorInfo::new(
                10069,
                "query datasource by type error".to_string(),
                "查询数据源信息错误".to_string(),
            )
            .new_with_data(data),
            Error::VerifyUdfFunctionNameError(data, _param) => AuroraErrorInfo::new(
                10070,
                "verify udf function name error".to_string(),
                "UDF函数名称验证错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10071,
                "delete udf function error".to_string(),
                "删除UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::AuthorizedFileResourceError(data, _param) => AuroraErrorInfo::new(
                10072,
                "authorized file resource error".to_string(),
                "授权资源文件错误".to_string(),
            )
            .new_with_data(data),
            Error::AuthorizeResourceTree(data, _param) => AuroraErrorInfo::new(
                10073,
                "authorize resource tree display error".to_string(),
                "授权资源目录树错误".to_string(),
            )
            .new_with_data(data),
            Error::UnauthorizedUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10074,
                "unauthorized udf function error".to_string(),
                "查询未授权UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::AuthorizedUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10075,
                "authorized udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateScheduleError(data, _param) => AuroraErrorInfo::new(
                10076,
                "create schedule error".to_string(),
                "创建调度配置错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateScheduleError(data, _param) => AuroraErrorInfo::new(
                10077,
                "update schedule error".to_string(),
                "更新调度配置错误".to_string(),
            )
            .new_with_data(data),
            Error::PublishScheduleOnlineError(data, _param) => AuroraErrorInfo::new(
                10078,
                "publish schedule online error".to_string(),
                "上线调度配置错误".to_string(),
            )
            .new_with_data(data),
            Error::OfflineScheduleError(data, _param) => AuroraErrorInfo::new(
                10079,
                "offline schedule error".to_string(),
                "下线调度配置错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryScheduleListPagingError(data, _param) => AuroraErrorInfo::new(
                10080,
                "query schedule list paging error".to_string(),
                "分页查询调度配置列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryScheduleListError(data, _param) => AuroraErrorInfo::new(
                10081,
                "query schedule list error".to_string(),
                "查询调度配置列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskListPagingError(data, _param) => AuroraErrorInfo::new(
                10082,
                "query task list paging error".to_string(),
                "分页查询任务列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskRecordListPagingError(data, _param) => AuroraErrorInfo::new(
                10083,
                "query task record list paging error".to_string(),
                "分页查询任务记录错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateTenantError(data, _param) => {
                AuroraErrorInfo::new(10084, "create tenant error".to_string(), "创建租户错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryTenantListPagingError(data, _param) => AuroraErrorInfo::new(
                10085,
                "query tenant list paging error".to_string(),
                "分页查询租户列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTenantListError(data, _param) => AuroraErrorInfo::new(
                10086,
                "query tenant list error".to_string(),
                "查询租户列表错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateTenantError(data, _param) => {
                AuroraErrorInfo::new(10087, "update tenant error".to_string(), "更新租户错误".to_string())
                    .new_with_data(data)
            }
            Error::DeleteTenantByIdError(data, _param) => AuroraErrorInfo::new(
                10088,
                "delete tenant by id error".to_string(),
                "删除租户错误".to_string(),
            )
            .new_with_data(data),
            Error::VerifyOsTenantCodeError(data, _param) => AuroraErrorInfo::new(
                10089,
                "verify os tenant code error".to_string(),
                "操作系统租户验证错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateUserError(data, _param) => {
                AuroraErrorInfo::new(10090, "create user error".to_string(), "创建用户错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryUserListPagingError(data, _param) => AuroraErrorInfo::new(
                10091,
                "query user list paging error".to_string(),
                "分页查询用户列表错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateUserError(data, _param) => {
                AuroraErrorInfo::new(10092, "update user error".to_string(), "更新用户错误".to_string())
                    .new_with_data(data)
            }
            Error::DeleteUserByIdError(data, _param) => {
                AuroraErrorInfo::new(10093, "delete user by id error".to_string(), "删除用户错误".to_string())
                    .new_with_data(data)
            }
            Error::GrantProjectError(data, _param) => {
                AuroraErrorInfo::new(10094, "grant project error".to_string(), "授权项目错误".to_string())
                    .new_with_data(data)
            }
            Error::GrantResourceError(data, _param) => {
                AuroraErrorInfo::new(10095, "grant resource error".to_string(), "授权资源错误".to_string())
                    .new_with_data(data)
            }
            Error::GrantUdfFunctionError(data, _param) => AuroraErrorInfo::new(
                10096,
                "grant udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            )
            .new_with_data(data),
            Error::GrantDatasourceError(data, _param) => AuroraErrorInfo::new(
                10097,
                "grant datasource error".to_string(),
                "授权数据源错误".to_string(),
            )
            .new_with_data(data),
            Error::GetUserInfoError(data, _param) => {
                AuroraErrorInfo::new(10098, "get user info error".to_string(), "获取用户信息错误".to_string())
                    .new_with_data(data)
            }
            Error::UserListError(data, _param) => {
                AuroraErrorInfo::new(10099, "user list error".to_string(), "查询用户列表错误".to_string())
                    .new_with_data(data)
            }
            Error::VerifyUsernameError(data, _param) => {
                AuroraErrorInfo::new(10100, "verify username error".to_string(), "用户名验证错误".to_string())
                    .new_with_data(data)
            }
            Error::UnauthorizedUserError(data, _param) => AuroraErrorInfo::new(
                10101,
                "unauthorized user error".to_string(),
                "查询未授权用户错误".to_string(),
            )
            .new_with_data(data),
            Error::AuthorizedUserError(data, _param) => AuroraErrorInfo::new(
                10102,
                "authorized user error".to_string(),
                "查询授权用户错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskInstanceLogError(data, _param) => AuroraErrorInfo::new(
                10103,
                "view task instance log error".to_string(),
                "查询任务实例日志错误".to_string(),
            )
            .new_with_data(data),
            Error::DownloadTaskInstanceLogFileError(data, _param) => AuroraErrorInfo::new(
                10104,
                "download task instance log file error".to_string(),
                "下载任务日志文件错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10105,
                "create process definition error".to_string(),
                "创建工作流错误".to_string(),
            )
            .new_with_data(data),
            Error::VerifyProcessDefinitionNameUniqueError(data, _param) => AuroraErrorInfo::new(
                10106,
                "verify process definition name unique error".to_string(),
                "工作流定义名称验证错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10107,
                "update process definition error".to_string(),
                "更新工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::ReleaseProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10108,
                "release process definition error".to_string(),
                "上线工作流错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryDetailOfProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10109,
                "query detail of process definition error".to_string(),
                "查询工作流详细信息错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryProcessDefinitionList(data, _param) => AuroraErrorInfo::new(
                10110,
                "query process definition list".to_string(),
                "查询工作流列表错误".to_string(),
            )
            .new_with_data(data),
            Error::EncapsulationTreeviewStructureError(data, _param) => AuroraErrorInfo::new(
                10111,
                "encapsulation treeview structure error".to_string(),
                "查询工作流树形图数据错误".to_string(),
            )
            .new_with_data(data),
            Error::GetTasksListByProcessDefinitionIdError(data, _param) => AuroraErrorInfo::new(
                10112,
                "get tasks list by process definition id error".to_string(),
                "查询工作流定义节点信息错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryProcessInstanceListPagingError(data, _param) => AuroraErrorInfo::new(
                10113,
                "query process instance list paging error".to_string(),
                "分页查询工作流实例列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskListByProcessInstanceIdError(data, _param) => AuroraErrorInfo::new(
                10114,
                "query task list by process instance id error".to_string(),
                "查询任务实例列表错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateProcessInstanceError(data, _param) => AuroraErrorInfo::new(
                10115,
                "update process instance error".to_string(),
                "更新工作流实例错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryProcessInstanceByIdError(data, _param) => AuroraErrorInfo::new(
                10116,
                "query process instance by id error".to_string(),
                "查询工作流实例错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteProcessInstanceByIdError(data, _param) => AuroraErrorInfo::new(
                10117,
                "delete process instance by id error".to_string(),
                "删除工作流实例错误".to_string(),
            )
            .new_with_data(data),
            Error::QuerySubProcessInstanceDetailInfoByTaskIdError(data, _param) => AuroraErrorInfo::new(
                10118,
                "query sub process instance detail info by task id error".to_string(),
                "查询子流程任务实例错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError(data, _param) => {
                AuroraErrorInfo::new(
                    10119,
                    "query parent process instance detail info by sub process instance id error".to_string(),
                    "查询子流程该工作流实例错误".to_string(),
                )
                .new_with_data(data)
            }
            Error::QueryProcessInstanceAllVariablesError(data, _param) => AuroraErrorInfo::new(
                10120,
                "query process instance all variables error".to_string(),
                "查询工作流自定义变量信息错误".to_string(),
            )
            .new_with_data(data),
            Error::EncapsulationProcessInstanceGanttStructureError(data, _param) => AuroraErrorInfo::new(
                10121,
                "encapsulation process instance gantt structure error".to_string(),
                "查询工作流实例甘特图数据错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryProcessDefinitionListPagingError(data, _param) => AuroraErrorInfo::new(
                10122,
                "query process definition list paging error".to_string(),
                "分页查询工作流定义列表错误".to_string(),
            )
            .new_with_data(data),
            Error::SignOutError(data, _param) => {
                AuroraErrorInfo::new(10123, "sign out error".to_string(), "退出错误".to_string()).new_with_data(data)
            }
            Error::OsTenantCodeHasAlreadyExists(data, _param) => AuroraErrorInfo::new(
                10124,
                "os tenant code has already exists".to_string(),
                "操作系统租户已存在".to_string(),
            )
            .new_with_data(data),
            Error::IpIsEmpty(data, _param) => {
                AuroraErrorInfo::new(10125, "ip is empty".to_string(), "IP地址不能为空".to_string()).new_with_data(data)
            }
            Error::ScheduleCronReleaseNeedNotChange(data, _param) => AuroraErrorInfo::new(
                10126,
                "schedule release is already {0}".to_string(),
                "调度配置上线错误[{0}]".to_string(),
            )
            .new_with_data(data),
            Error::CreateQueueError(data, _param) => {
                AuroraErrorInfo::new(10127, "create queue error".to_string(), "创建队列错误".to_string())
                    .new_with_data(data)
            }
            Error::QueueNotExist(data, _param) => AuroraErrorInfo::new(
                10128,
                "queue {0} not exists".to_string(),
                "队列ID[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::QueueValueExist(data, _param) => AuroraErrorInfo::new(
                10129,
                "queue value {0} already exists".to_string(),
                "队列值[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::QueueNameExist(data, error_param) => {
                error!("queue name {} already exists", data);
                AuroraErrorInfo::new(
                    10130,
                    "queue name {0} already exists".to_string(),
                    "队列名称[{0}]已存在".to_string(),
                )
                .new_with_data(data)
                .parse(error_param)
            }
            Error::UpdateQueueError(data, _param) => {
                AuroraErrorInfo::new(10131, "update queue error".to_string(), "更新队列信息错误".to_string())
                    .new_with_data(data)
            }
            Error::NeedNotUpdateQueue(data, _param) => AuroraErrorInfo::new(
                10132,
                "need not update queue".to_string(),
                "无需更新队列信息".to_string(),
            )
            .new_with_data(data),
            Error::VerifyQueueError(data, _param) => {
                AuroraErrorInfo::new(10133, "verify queue error".to_string(), "验证队列信息错误".to_string())
                    .new_with_data(data)
            }
            Error::NameNull(data, _param) => {
                AuroraErrorInfo::new(10134, "name must be not null".to_string(), "名称不能为空".to_string())
                    .new_with_data(data)
            }
            Error::NameExist(data, _param) => AuroraErrorInfo::new(
                10135,
                "name {0} already exists".to_string(),
                "名称[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::SaveError(data, _param) => {
                AuroraErrorInfo::new(10136, "save error".to_string(), "保存错误".to_string()).new_with_data(data)
            }
            Error::DeleteProjectErrorDefinesNotNull(data, _param) => AuroraErrorInfo::new(
                10117,
                "please delete the process definitions in project first!".to_string(),
                "请先删除全部工作流定义".to_string(),
            )
            .new_with_data(data),
            Error::BatchDeleteProcessInstanceByIdsError(data, _param) => AuroraErrorInfo::new(
                10138,
                "batch delete process instance by ids {0} error".to_string(),
                "批量删除工作流实例错误: {0}".to_string(),
            )
            .new_with_data(data),
            Error::PreviewScheduleError(data, _param) => AuroraErrorInfo::new(
                10139,
                "preview schedule error".to_string(),
                "预览调度配置错误".to_string(),
            )
            .new_with_data(data),
            Error::ParseToCronExpressionError(data, _param) => AuroraErrorInfo::new(
                10140,
                "parse cron to cron expression error".to_string(),
                "解析调度表达式错误".to_string(),
            )
            .new_with_data(data),
            Error::ScheduleStartTimeEndTimeSame(data, _param) => AuroraErrorInfo::new(
                10141,
                "The start time must not be the same as the end".to_string(),
                "开始时间不能和结束时间一样".to_string(),
            )
            .new_with_data(data),
            Error::DeleteTenantByIdFail(data, _param) => AuroraErrorInfo::new(
                10142,
                "delete tenant by id fail:for there are {0} process instances in executing using it".to_string(),
                "删除租户失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            )
            .new_with_data(data),
            Error::DeleteTenantByIdFailDefines(data, _param) => AuroraErrorInfo::new(
                10143,
                "delete tenant by id fail:for there are {0} process definitions using it".to_string(),
                "删除租户失败，有[{0}]个工作流定义正在使用".to_string(),
            )
            .new_with_data(data),
            Error::DeleteTenantByIdFailUsers(data, _param) => AuroraErrorInfo::new(
                10144,
                "delete tenant by id fail: for there are {0} users using it".to_string(),
                "删除租户失败，有[{0}]个用户正在使用".to_string(),
            )
            .new_with_data(data),
            Error::DeleteWorkerGroupByIdFail(data, _param) => AuroraErrorInfo::new(
                10145,
                "delete worker group by id failfor there are {0} process instances in executing using it".to_string(),
                "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            )
            .new_with_data(data),
            Error::QueryWorkerGroupFail(data, _param) => AuroraErrorInfo::new(
                10146,
                "query worker group fail ".to_string(),
                "查询worker分组失败".to_string(),
            )
            .new_with_data(data),
            Error::DeleteWorkerGroupFail(data, _param) => AuroraErrorInfo::new(
                10147,
                "delete worker group fail ".to_string(),
                "删除worker分组失败".to_string(),
            )
            .new_with_data(data),
            Error::UserDisabled(data, _param) => AuroraErrorInfo::new(
                10148,
                "The current user is disabled".to_string(),
                "当前用户已停用".to_string(),
            )
            .new_with_data(data),
            Error::CopyProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10149,
                "copy process definition from {0} to {1} error : {2}".to_string(),
                "从{0}复制工作流到{1}错误 : {2}".to_string(),
            )
            .new_with_data(data),
            Error::MoveProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10150,
                "move process definition from {0} to {1} error : {2}".to_string(),
                "从{0}移动工作流到{1}错误 : {2}".to_string(),
            )
            .new_with_data(data),
            Error::SwitchProcessDefinitionVersionError(data, _param) => AuroraErrorInfo::new(
                10151,
                "Switch process definition version error".to_string(),
                "切换工作流版本出错".to_string(),
            )
            .new_with_data(data),
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10152,
                "Switch process definition version error: not exists process definition: [process definition id {0}]"
                    .to_string(),
                "切换工作流版本出错：工作流不存在，[工作流id {0}]".to_string(),
            )
            .new_with_data(data),
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError(data, _param) => {
                AuroraErrorInfo::new(
                    10153,
                    "Switch process defi:nition version error: not exists process definition version: [process \
                     definition id {0}] [version number {1}]"
                        .to_string(),
                    "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]".to_string(),
                )
                .new_with_data(data)
            }
            Error::QueryProcessDefinitionVersionsError(data, _param) => AuroraErrorInfo::new(
                10154,
                "query process definition versions error".to_string(),
                "查询工作流历史版本信息出错".to_string(),
            )
            .new_with_data(data),
            Error::DeleteProcessDefinitionVersionError(data, _param) => AuroraErrorInfo::new(
                10156,
                "delete process definition version error".to_string(),
                "删除工作流历史版本出错".to_string(),
            )
            .new_with_data(data),
            Error::QueryUserCreatedProjectError(data, _param) => AuroraErrorInfo::new(
                10157,
                "query user created project error error".to_string(),
                "查询用户创建的项目错误".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefinitionCodesIsEmpty(data, _param) => AuroraErrorInfo::new(
                10158,
                "process definition codes is empty".to_string(),
                "工作流CODES不能为空".to_string(),
            )
            .new_with_data(data),
            Error::BatchCopyProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10159,
                "batch copy process definition error".to_string(),
                "复制工作流错误".to_string(),
            )
            .new_with_data(data),
            Error::BatchMoveProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                10160,
                "batch move process definition error".to_string(),
                "移动工作流错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryWorkflowLineageError(data, _param) => AuroraErrorInfo::new(
                10161,
                "query workflow lineage error".to_string(),
                "查询血缘失败".to_string(),
            )
            .new_with_data(data),
            Error::QueryAuthorizedAndUserCreatedProjectError(data, _param) => AuroraErrorInfo::new(
                10162,
                "query authorized and user created project error error".to_string(),
                "查询授权的和用户创建的项目错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteProcessDefinitionByCodeFail(data, _param) => AuroraErrorInfo::new(
                10163,
                "delete process definition by code fail.to_string(), for there are {0} process instances in executing \
                 using it"
                    .to_string(),
                "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            )
            .new_with_data(data),
            Error::CheckOsTenantCodeError(data, _param) => AuroraErrorInfo::new(
                10164,
                "Tenant code invalid.to_string(), should follow linux's users naming conventions".to_string(),
                "非法的租户名，需要遵守 Linux 用户命名规范".to_string(),
            )
            .new_with_data(data),
            Error::ForceTaskSuccessError(data, _param) => AuroraErrorInfo::new(
                10165,
                "force task success error".to_string(),
                "强制成功任务实例错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskInstanceStateOperationError(data, _param) => AuroraErrorInfo::new(
                10166,
                "the status of task instance {0} is {1}.to_string(),Cannot perform force success operation".to_string(),
                "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceTypeNotExist(data, _param) => AuroraErrorInfo::new(
                10167,
                "data source type not exist".to_string(),
                "数据源类型不存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefinitionNameExist(data, _param) => AuroraErrorInfo::new(
                10168,
                "process definition name {0} already exists".to_string(),
                "工作流定义名称[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceDbTypeIllegal(data, _param) => AuroraErrorInfo::new(
                10169,
                "datasource type illegal".to_string(),
                "数据源类型参数不合法".to_string(),
            )
            .new_with_data(data),
            Error::DatasourcePortIllegal(data, _param) => AuroraErrorInfo::new(
                10170,
                "datasource port illegal".to_string(),
                "数据源端口参数不合法".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceOtherParamsIllegal(data, _param) => AuroraErrorInfo::new(
                10171,
                "datasource other params illegal".to_string(),
                "数据源其他参数不合法".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceNameIllegal(data, _param) => AuroraErrorInfo::new(
                10172,
                "datasource name illegal".to_string(),
                "数据源名称不合法".to_string(),
            )
            .new_with_data(data),
            Error::DatasourceHostIllegal(data, _param) => AuroraErrorInfo::new(
                10173,
                "datasource host illegal".to_string(),
                "数据源HOST不合法".to_string(),
            )
            .new_with_data(data),
            Error::DeleteWorkerGroupNotExist(data, _param) => AuroraErrorInfo::new(
                10174,
                "delete worker group not exist ".to_string(),
                "删除worker分组不存在".to_string(),
            )
            .new_with_data(data),
            Error::CreateWorkerGroupForbiddenInDocker(data, _param) => AuroraErrorInfo::new(
                10175,
                "create worker group forbidden in docker ".to_string(),
                "创建worker分组在docker中禁止".to_string(),
            )
            .new_with_data(data),
            Error::DeleteWorkerGroupForbiddenInDocker(data, _param) => AuroraErrorInfo::new(
                10176,
                "delete worker group forbidden in docker ".to_string(),
                "删除worker分组在docker中禁止".to_string(),
            )
            .new_with_data(data),
            Error::WorkerAddressInvalid(data, _param) => AuroraErrorInfo::new(
                10177,
                "worker address {0} invalid".to_string(),
                "worker地址[{0}]无效".to_string(),
            )
            .new_with_data(data),
            Error::QueryWorkerAddressListFail(data, _param) => AuroraErrorInfo::new(
                10178,
                "query worker address list fail ".to_string(),
                "查询worker地址列表失败".to_string(),
            )
            .new_with_data(data),
            Error::TransformProjectOwnership(data, _param) => AuroraErrorInfo::new(
                10179,
                "Please transform project ownership [{0}]".to_string(),
                "请先转移项目所有权[{0}]".to_string(),
            )
            .new_with_data(data),
            Error::QueryAlertGroupError(data, _param) => AuroraErrorInfo::new(
                10180,
                "query alert group error".to_string(),
                "查询告警组错误".to_string(),
            )
            .new_with_data(data),
            Error::CurrentLoginUserTenantNotExist(data, _param) => AuroraErrorInfo::new(
                10181,
                "the tenant of the currently login user is not specified".to_string(),
                "未指定当前登录用户的租户".to_string(),
            )
            .new_with_data(data),
            Error::RevokeProjectError(data, _param) => AuroraErrorInfo::new(
                10182,
                "revoke project error".to_string(),
                "撤销项目授权错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryAuthorizedUser(data, _param) => AuroraErrorInfo::new(
                10183,
                "query authorized user error".to_string(),
                "查询拥有项目权限的用户错误".to_string(),
            )
            .new_with_data(data),
            Error::ProjectNotExist(data, _param) => AuroraErrorInfo::new(
                10190,
                "This project was not found. Please refresh page.".to_string(),
                "该项目不存在".to_string(),
            )
            .new_with_data(data),

            Error::TaskInstanceHostIsNull(data, _param) => AuroraErrorInfo::new(
                10191,
                "task instance host is null ".to_string(),
                "任务实例host为空".to_string(),
            )
            .new_with_data(data),
            Error::QueryExecutingWorkflowError(data, _param) => AuroraErrorInfo::new(
                10192,
                "query executing workflow error".to_string(),
                "查询运行的工作流实例错误".to_string(),
            )
            .new_with_data(data),
            Error::UdfFunctionNotExist(data, _param) => {
                AuroraErrorInfo::new(20001, "UDF function not found".to_string(), "UDF函数不存在".to_string())
                    .new_with_data(data)
            }
            Error::UdfFunctionExists(data, _param) => AuroraErrorInfo::new(
                20002,
                "UDF function already exists".to_string(),
                "UDF函数已存在".to_string(),
            )
            .new_with_data(data),
            Error::ResourceNotExist(data, _param) => {
                AuroraErrorInfo::new(20004, "resource not exist".to_string(), "资源不存在".to_string())
                    .new_with_data(data)
            }
            Error::ResourceExist(data, _param) => {
                AuroraErrorInfo::new(20005, "resource already exists".to_string(), "资源已存在".to_string())
                    .new_with_data(data)
            }
            Error::ResourceSuffixNotSupportView(data, _param) => AuroraErrorInfo::new(
                20006,
                "resource suffix do not support online viewing".to_string(),
                "资源文件后缀不支持查看".to_string(),
            )
            .new_with_data(data),
            Error::ResourceSizeExceedLimit(data, _param) => AuroraErrorInfo::new(
                20007,
                "upload resource file size exceeds limit".to_string(),
                "上传资源文件大小超过限制".to_string(),
            )
            .new_with_data(data),
            Error::ResourceSuffixForbidChange(data, _param) => AuroraErrorInfo::new(
                20008,
                "resource suffix not allowed to be modified".to_string(),
                "资源文件后缀不支持修改".to_string(),
            )
            .new_with_data(data),
            Error::UdfResourceSuffixNotJar(data, _param) => AuroraErrorInfo::new(
                20009,
                "UDF resource suffix name must be jar".to_string(),
                "UDF资源文件后缀名只支持[jar]".to_string(),
            )
            .new_with_data(data),
            Error::HdfsCopyFail(data, _param) => AuroraErrorInfo::new(
                20010,
                "hdfs copy {0} -> {1} fail".to_string(),
                "hdfs复制失败：[{0}] -> [{1}]".to_string(),
            )
            .new_with_data(data),
            Error::ResourceFileExist(data, _param) => AuroraErrorInfo::new(
                20011,
                "resource file {0} already exists !".to_string(),
                "资源文件[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::ResourceFileNotExist(data, _param) => AuroraErrorInfo::new(
                20012,
                "resource file {0} not exists !".to_string(),
                "资源文件[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::UdfResourceIsBound(data, _param) => AuroraErrorInfo::new(
                20013,
                "udf resource file is bound by UDF functions:{0}".to_string(),
                "udf函数绑定了资源文件[{0}]".to_string(),
            )
            .new_with_data(data),
            Error::ResourceIsUsed(data, _param) => AuroraErrorInfo::new(
                20014,
                "resource file is used by process definition".to_string(),
                "资源文件被上线的流程定义使用了".to_string(),
            )
            .new_with_data(data),
            Error::ParentResourceNotExist(data, _param) => AuroraErrorInfo::new(
                20015,
                "parent resource not exist".to_string(),
                "父资源文件不存在".to_string(),
            )
            .new_with_data(data),
            Error::ResourceNotExistOrNoPermission(data, _param) => AuroraErrorInfo::new(
                20016,
                "resource not exist or no permission:please view the task node and remove error resource".to_string(),
                "请检查任务节点并移除无权限或者已删除的资源".to_string(),
            )
            .new_with_data(data),
            Error::ResourceIsAuthorized(data, _param) => AuroraErrorInfo::new(
                20017,
                "resource is authorized to user {0}:suffix not allowed to be modified".to_string(),
                "资源文件已授权其他用户[{0}]".to_string(),
            )
            .new_with_data(data),
            Error::UserNoOperationPerm(data, _param) => AuroraErrorInfo::new(
                30001,
                "user has no operation privilege".to_string(),
                "当前用户没有操作权限".to_string(),
            )
            .new_with_data(data),
            Error::UserNoOperationProjectPerm(data, _param) => AuroraErrorInfo::new(
                30002,
                "user {0} is not has project {1} permission".to_string(),
                "当前用户[{0}]没有[{1}]项目的操作权限".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceNotExist(data, _param) => AuroraErrorInfo::new(
                50001,
                "process instance {0} does not exist".to_string(),
                "工作流实例[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceExist(data, _param) => AuroraErrorInfo::new(
                50002,
                "process instance {0} already exists".to_string(),
                "工作流实例[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefineNotExist(data, _param) => AuroraErrorInfo::new(
                50003,
                "process definition {0} does not exist".to_string(),
                "工作流定义[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefineNotRelease(data, _param) => AuroraErrorInfo::new(
                50004,
                "process definition {0} process version {1} not online".to_string(),
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态".to_string(),
            )
            .new_with_data(data),
            Error::SubProcessDefineNotRelease(data, _param) => AuroraErrorInfo::new(
                50004,
                "exist sub process definition not online".to_string(),
                "存在子工作流定义不是上线状态".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceAlreadyChanged(data, _param) => AuroraErrorInfo::new(
                50005,
                "the status of process instance {0} is already {1}".to_string(),
                "工作流实例[{0}]的状态已经是[{1}]".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceStateOperationError(data, _param) => AuroraErrorInfo::new(
                50006,
                "the status of process instance {0} is {1}.to_string(),Cannot perform the operation".to_string(),
                "工作流实例[{0}]的状态是[{1}]，无法执行该操作".to_string(),
            )
            .new_with_data(data),
            Error::SubProcessInstanceNotExist(data, _param) => AuroraErrorInfo::new(
                50007,
                "the task belong to process instance does not exist".to_string(),
                "子工作流实例不存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefineNotAllowedEdit(data, _param) => AuroraErrorInfo::new(
                50008,
                "process definition {0} does not allow edit".to_string(),
                "工作流定义[{0}]不允许修改".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceExecutingCommand(data, _param) => AuroraErrorInfo::new(
                50009,
                "process instance {0} is executing command".to_string(),
                "工作流实例[{0}]正在执行命令".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceNotSubProcessInstance(data, _param) => AuroraErrorInfo::new(
                50010,
                "process instance {0} is not sub process instance".to_string(),
                "工作流实例[{0}]不是子工作流实例".to_string(),
            )
            .new_with_data(data),
            Error::TaskInstanceStateCountError(data, _param) => AuroraErrorInfo::new(
                50011,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            )
            .new_with_data(data),
            Error::CountProcessInstanceStateError(data, _param) => AuroraErrorInfo::new(
                50012,
                "count process instance state error".to_string(),
                "查询各状态流程实例数错误".to_string(),
            )
            .new_with_data(data),
            Error::CountProcessDefinitionUserError(data, _param) => AuroraErrorInfo::new(
                50013,
                "count process definition user error".to_string(),
                "查询各用户流程定义数错误".to_string(),
            )
            .new_with_data(data),
            Error::StartProcessInstanceError(data, _param) => AuroraErrorInfo::new(
                50014,
                "start process instance error".to_string(),
                "运行工作流实例错误".to_string(),
            )
            .new_with_data(data),
            Error::BatchStartProcessInstanceError(data, _param) => AuroraErrorInfo::new(
                50014,
                "batch start process instance error: {0}".to_string(),
                "批量运行工作流实例错误: {0}".to_string(),
            )
            .new_with_data(data),
            Error::ProcessInstanceError(data, _param) => AuroraErrorInfo::new(
                50014,
                "process instance delete error: {0}".to_string(),
                "工作流实例删除[{0}]错误".to_string(),
            )
            .new_with_data(data),
            Error::ExecuteProcessInstanceError(data, _param) => AuroraErrorInfo::new(
                50015,
                "execute process instance error".to_string(),
                "操作工作流实例错误".to_string(),
            )
            .new_with_data(data),
            Error::CheckProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                50016,
                "check process definition error".to_string(),
                "工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryRecipientsAndCopyersByProcessDefinitionError(data, _param) => AuroraErrorInfo::new(
                50017,
                "query recipients and copyers by process definition error".to_string(),
                "查询收件人和抄送人错误".to_string(),
            )
            .new_with_data(data),
            Error::DataIsNotValid(data, _param) => {
                AuroraErrorInfo::new(50017, "data {0} not valid".to_string(), "数据[{0}]无效".to_string())
                    .new_with_data(data)
            }
            Error::DataIsNull(data, _param) => {
                AuroraErrorInfo::new(50018, "data {0} is null".to_string(), "数据[{0}]不能为空".to_string())
                    .new_with_data(data)
            }
            Error::ProcessNodeHasCycle(data, _param) => AuroraErrorInfo::new(
                50019,
                "process node has cycle".to_string(),
                "流程节点间存在循环依赖".to_string(),
            )
            .new_with_data(data),
            Error::ProcessNodeSParameterInvalid(data, _param) => AuroraErrorInfo::new(
                50020,
                "process node {0} parameter invalid".to_string(),
                "流程节点[{0}]参数无效".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefineStateOnline(data, _param) => AuroraErrorInfo::new(
                50021,
                "process definition [{0}] is already online".to_string(),
                "工作流定义[{0}]已上线".to_string(),
            )
            .new_with_data(data),
            Error::DeleteProcessDefineByCodeError(data, _param) => AuroraErrorInfo::new(
                50022,
                "delete process definition by code error".to_string(),
                "删除工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::ScheduleCronStateOnline(data, _param) => AuroraErrorInfo::new(
                50023,
                "the status of schedule {0} is already online".to_string(),
                "调度配置[{0}]已上线".to_string(),
            )
            .new_with_data(data),
            Error::DeleteScheduleCronByIdError(data, _param) => AuroraErrorInfo::new(
                50024,
                "delete schedule by id error".to_string(),
                "删除调度配置错误".to_string(),
            )
            .new_with_data(data),
            Error::BatchDeleteProcessDefineError(data, _param) => AuroraErrorInfo::new(
                50025,
                "batch delete process definition error".to_string(),
                "批量删除工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::BatchDeleteProcessDefineByCodesError(data, _param) => AuroraErrorInfo::new(
                50026,
                "batch delete process definition by codes {0} error".to_string(),
                "批量删除工作流定义[{0}]错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteProcessDefineByCodesError(data, _param) => AuroraErrorInfo::new(
                50026,
                "delete process definition by codes {0} error".to_string(),
                "删除工作流定义[{0}]错误".to_string(),
            )
            .new_with_data(data),
            Error::TenantNotSuitable(data, _param) => AuroraErrorInfo::new(
                50027,
                "there is not any tenant suitable: please choose a tenant available.".to_string(),
                "没有合适的租户，请选择可用的租户".to_string(),
            )
            .new_with_data(data),
            Error::ExportProcessDefineByIdError(data, _param) => AuroraErrorInfo::new(
                50028,
                "export process definition by id error".to_string(),
                "导出工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::BatchExportProcessDefineByIdsError(data, _param) => AuroraErrorInfo::new(
                50028,
                "batch export process definition by ids error".to_string(),
                "批量导出工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::ImportProcessDefineError(data, _param) => AuroraErrorInfo::new(
                50029,
                "import process definition error".to_string(),
                "导入工作流定义错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskDefineNotExist(data, _param) => AuroraErrorInfo::new(
                50030,
                "task definition [{0}] does not exist".to_string(),
                "任务定义[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::CreateProcessTaskRelationError(data, _param) => AuroraErrorInfo::new(
                50032,
                "create process task relation error".to_string(),
                "创建工作流任务关系错误".to_string(),
            )
            .new_with_data(data),
            Error::ProcessTaskRelationNotExist(data, _param) => AuroraErrorInfo::new(
                50033,
                "process task relation [{0}] does not exist".to_string(),
                "工作流任务关系[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessTaskRelationExist(data, _param) => AuroraErrorInfo::new(
                50034,
                "process task relation is already exist  processCode:[{0}]".to_string(),
                "工作流任务关系已存在".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDagIsEmpty(data, _param) => {
                AuroraErrorInfo::new(50035, "process dag is empty".to_string(), "工作流dag是空".to_string())
                    .new_with_data(data)
            }
            Error::CheckProcessTaskRelationError(data, _param) => AuroraErrorInfo::new(
                50036,
                "check process task relation error".to_string(),
                "工作流任务关系参数错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateTaskDefinitionError(data, _param) => AuroraErrorInfo::new(
                50037,
                "create task definition error".to_string(),
                "创建任务错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateTaskDefinitionError(data, _param) => AuroraErrorInfo::new(
                50038,
                "update task definition error".to_string(),
                "更新任务定义错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskDefinitionVersionsError(data, _param) => AuroraErrorInfo::new(
                50039,
                "query task definition versions error".to_string(),
                "查询任务历史版本信息出错".to_string(),
            )
            .new_with_data(data),
            Error::SwitchTaskDefinitionVersionError(data, _param) => AuroraErrorInfo::new(
                50040,
                "Switch task definition version error".to_string(),
                "切换任务版本出错".to_string(),
            )
            .new_with_data(data),
            Error::DeleteTaskDefinitionVersionError(data, _param) => AuroraErrorInfo::new(
                50041,
                "delete task definition version error".to_string(),
                "删除任务历史版本出错".to_string(),
            )
            .new_with_data(data),
            Error::DeleteTaskDefineByCodeError(data, _param) => AuroraErrorInfo::new(
                50042,
                "delete task definition by code error".to_string(),
                "删除任务定义错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryDetailOfTaskDefinitionError(data, _param) => AuroraErrorInfo::new(
                50043,
                "query detail of task definition error".to_string(),
                "查询任务详细信息错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskDefinitionListPagingError(data, _param) => AuroraErrorInfo::new(
                50044,
                "query task definition list paging error".to_string(),
                "分页查询任务定义列表错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskDefinitionNameExisted(data, _param) => AuroraErrorInfo::new(
                50045,
                "task definition name [{0}] already exists".to_string(),
                "任务定义名称[{0}]已经存在".to_string(),
            )
            .new_with_data(data),
            Error::ReleaseTaskDefinitionError(data, _param) => AuroraErrorInfo::new(
                50046,
                "release task definition error".to_string(),
                "上线任务错误".to_string(),
            )
            .new_with_data(data),
            Error::MoveProcessTaskRelationError(data, _param) => AuroraErrorInfo::new(
                50047,
                "move process task relation error".to_string(),
                "移动任务到其他工作流错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteTaskProcessRelationError(data, _param) => AuroraErrorInfo::new(
                50048,
                "delete process task relation error".to_string(),
                "删除工作流任务关系错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskProcessRelationError(data, _param) => AuroraErrorInfo::new(
                50049,
                "query process task relation error".to_string(),
                "查询工作流任务关系错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskDefineStateOnline(data, _param) => AuroraErrorInfo::new(
                50050,
                "task definition [{0}] is already online".to_string(),
                "任务定义[{0}]已上线".to_string(),
            )
            .new_with_data(data),
            Error::TaskHasDownstream(data, _param) => AuroraErrorInfo::new(
                50051,
                "Task exists downstream [{0}] dependence".to_string(),
                "任务存在下游[{0}]依赖".to_string(),
            )
            .new_with_data(data),
            Error::TaskHasUpstream(data, _param) => AuroraErrorInfo::new(
                50052,
                "Task [{0}] exists upstream dependence".to_string(),
                "任务[{0}]存在上游依赖".to_string(),
            )
            .new_with_data(data),
            Error::MainTableUsingVersion(data, _param) => AuroraErrorInfo::new(
                50053,
                "the version that the master table is using".to_string(),
                "主表正在使用该版本".to_string(),
            )
            .new_with_data(data),
            Error::ProjectProcessNotMatch(data, _param) => AuroraErrorInfo::new(
                50054,
                "the project and the process is not match".to_string(),
                "项目和工作流不匹配".to_string(),
            )
            .new_with_data(data),
            Error::DeleteEdgeError(data, _param) => AuroraErrorInfo::new(
                50055,
                "delete edge error".to_string(),
                "删除工作流任务连接线错误".to_string(),
            )
            .new_with_data(data),
            Error::NotSupportUpdateTaskDefinition(data, _param) => AuroraErrorInfo::new(
                50056,
                "task state does not support modification".to_string(),
                "当前任务不支持修改".to_string(),
            )
            .new_with_data(data),
            Error::NotSupportCopyTaskType(data, _param) => AuroraErrorInfo::new(
                50057,
                "task type [{0}] does not support copy".to_string(),
                "不支持复制的任务类型[{0}]".to_string(),
            )
            .new_with_data(data),
            Error::HdfsNotStartup(data, _param) => {
                AuroraErrorInfo::new(60001, "hdfs not startup".to_string(), "hdfs未启用".to_string())
                    .new_with_data(data)
            }
            Error::StorageNotStartup(data, _param) => {
                AuroraErrorInfo::new(60002, "storage not startup".to_string(), "存储未启用".to_string())
                    .new_with_data(data)
            }
            Error::S3CannotRename(data, _param) => AuroraErrorInfo::new(
                60003,
                "directory cannot be renamed".to_string(),
                "S3无法重命名文件夹".to_string(),
            )
            .new_with_data(data),
            Error::QueryDatabaseStateError(data, _param) => AuroraErrorInfo::new(
                70001,
                "query database state error".to_string(),
                "查询数据库状态错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateAccessTokenError(data, _param) => AuroraErrorInfo::new(
                70010,
                "create access token error".to_string(),
                "创建访问token错误".to_string(),
            )
            .new_with_data(data),
            Error::GenerateTokenError(data, _param) => {
                AuroraErrorInfo::new(70011, "generate token error".to_string(), "生成token错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryAccesstokenListPagingError(data, _param) => AuroraErrorInfo::new(
                70012,
                "query access token list paging error".to_string(),
                "分页查询访问token列表错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateAccessTokenError(data, _param) => AuroraErrorInfo::new(
                70013,
                "update access token error".to_string(),
                "更新访问token错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteAccessTokenError(data, _param) => AuroraErrorInfo::new(
                70014,
                "delete access token error".to_string(),
                "删除访问token错误".to_string(),
            )
            .new_with_data(data),
            Error::AccessTokenNotExist(data, _param) => AuroraErrorInfo::new(
                70015,
                "access token not exist".to_string(),
                "访问token不存在".to_string(),
            )
            .new_with_data(data),
            Error::QueryAccesstokenByUserError(data, _param) => AuroraErrorInfo::new(
                70016,
                "query access token by user error".to_string(),
                "查询访问指定用户的token错误".to_string(),
            )
            .new_with_data(data),
            Error::CommandStateCountError(data, _param) => AuroraErrorInfo::new(
                80001,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            )
            .new_with_data(data),
            Error::NegativeSizeNumberError(data, _param) => {
                AuroraErrorInfo::new(80002, "query size number error".to_string(), "查询size错误".to_string())
                    .new_with_data(data)
            }
            Error::StartTimeBiggerThanEndTimeError(data, _param) => AuroraErrorInfo::new(
                80003,
                "start time bigger than end time error".to_string(),
                "开始时间在结束时间之后错误".to_string(),
            )
            .new_with_data(data),
            Error::QueueCountError(data, _param) => {
                AuroraErrorInfo::new(90001, "queue count error".to_string(), "查询队列数据错误".to_string())
                    .new_with_data(data)
            }
            Error::KerberosStartupState(data, _param) => AuroraErrorInfo::new(
                100001,
                "get kerberos startup state error".to_string(),
                "获取kerberos启动状态错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryAuditLogListPaging(data, _param) => AuroraErrorInfo::new(
                10057,
                "query audit log list paging".to_string(),
                "分页查询日志列表错误".to_string(),
            )
            .new_with_data(data),
            Error::PluginNotAUiComponent(data, _param) => AuroraErrorInfo::new(
                110001,
                "query plugin error: this plugin has no UI component".to_string(),
                "查询插件错误，此插件无UI组件".to_string(),
            )
            .new_with_data(data),
            Error::QueryPluginsResultIsNull(data, _param) => AuroraErrorInfo::new(
                110002,
                "query alarm plugins result is empty:please check the startup status of the alarm component and \
                 confirm that the relevant alarm plugin is successfully registered"
                    .to_string(),
                "查询告警插件为空".to_string(),
            )
            .new_with_data(data),
            Error::QueryPluginsError(data, _param) => {
                AuroraErrorInfo::new(110003, "query plugins error".to_string(), "查询插件错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryPluginDetailResultIsNull(data, _param) => AuroraErrorInfo::new(
                110004,
                "query plugin detail result is null".to_string(),
                "查询插件详情结果为空".to_string(),
            )
            .new_with_data(data),
            Error::UpdateAlertPluginInstanceError(data, _param) => AuroraErrorInfo::new(
                110005,
                "update alert plugin instance error".to_string(),
                "更新告警组和告警组插件实例错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteAlertPluginInstanceError(data, _param) => AuroraErrorInfo::new(
                110006,
                "delete alert plugin instance error".to_string(),
                "删除告警组和告警组插件实例错误".to_string(),
            )
            .new_with_data(data),
            Error::GetAlertPluginInstanceError(data, _param) => AuroraErrorInfo::new(
                110007,
                "get alert plugin instance error".to_string(),
                "获取告警组和告警组插件实例错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateAlertPluginInstanceError(data, _param) => AuroraErrorInfo::new(
                110008,
                "create alert plugin instance error".to_string(),
                "创建告警组和告警组插件实例错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryAllAlertPluginInstanceError(data, _param) => AuroraErrorInfo::new(
                110009,
                "query all alert plugin instance error".to_string(),
                "查询所有告警实例失败".to_string(),
            )
            .new_with_data(data),
            Error::PluginInstanceAlreadyExit(data, _param) => AuroraErrorInfo::new(
                110010,
                "plugin instance already exit".to_string(),
                "该告警插件实例已存在".to_string(),
            )
            .new_with_data(data),
            Error::ListPagingAlertPluginInstanceError(data, _param) => AuroraErrorInfo::new(
                110011,
                "query plugin instance page error".to_string(),
                "分页查询告警实例失败".to_string(),
            )
            .new_with_data(data),
            Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated(data, _param) => AuroraErrorInfo::new(
                110012,
                "failed to delete the alert instance there is an alarm group associated with this alert instance"
                    .to_string(),
                "删除告警实例失败，存在与此告警实例关联的警报组".to_string(),
            )
            .new_with_data(data),
            Error::ProcessDefinitionVersionIsUsed(data, _param) => AuroraErrorInfo::new(
                110013,
                "this process definition version is used".to_string(),
                "此工作流定义版本被使用".to_string(),
            )
            .new_with_data(data),
            Error::CreateEnvironmentError(data, _param) => AuroraErrorInfo::new(
                120001,
                "create environment error".to_string(),
                "创建环境失败".to_string(),
            )
            .new_with_data(data),
            Error::EnvironmentNameExists(data, _param) => AuroraErrorInfo::new(
                120002,
                "this environment name [{0}] already exists".to_string(),
                "环境名称[{0}]已经存在".to_string(),
            )
            .new_with_data(data),
            Error::EnvironmentNameIsNull(data, _param) => AuroraErrorInfo::new(
                120003,
                "this environment name shouldn't be empty.".to_string(),
                "环境名称不能为空".to_string(),
            )
            .new_with_data(data),
            Error::EnvironmentConfigIsNull(data, _param) => AuroraErrorInfo::new(
                120004,
                "this environment config shouldn't be empty.".to_string(),
                "环境配置信息不能为空".to_string(),
            )
            .new_with_data(data),
            Error::UpdateEnvironmentError(data, _param) => AuroraErrorInfo::new(
                120005,
                "update environment [{0}] info error".to_string(),
                "更新环境[{0}]信息失败".to_string(),
            )
            .new_with_data(data),
            Error::DeleteEnvironmentError(data, _param) => AuroraErrorInfo::new(
                120006,
                "delete environment error".to_string(),
                "删除环境信息失败".to_string(),
            )
            .new_with_data(data),
            Error::DeleteEnvironmentRelatedTaskExists(data, _param) => AuroraErrorInfo::new(
                120007,
                "delete environment error, related task exists".to_string(),
                "删除环境信息失败，存在关联任务".to_string(),
            )
            .new_with_data(data),
            Error::QueryEnvironmentByNameError(data, _param) => AuroraErrorInfo::new(
                1200008,
                "not found environment [{0}] ".to_string(),
                "查询环境名称[{0}]信息不存在".to_string(),
            )
            .new_with_data(data),
            Error::QueryEnvironmentByCodeError(data, _param) => AuroraErrorInfo::new(
                1200009,
                "not found environment [{0}] ".to_string(),
                "查询环境编码[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::QueryEnvironmentError(data, _param) => AuroraErrorInfo::new(
                1200010,
                "login user query environment error".to_string(),
                "分页查询环境列表错误".to_string(),
            )
            .new_with_data(data),
            Error::VerifyEnvironmentError(data, _param) => AuroraErrorInfo::new(
                1200011,
                "verify environment error".to_string(),
                "验证环境信息错误".to_string(),
            )
            .new_with_data(data),
            Error::GetRuleFormCreateJsonError(data, _param) => AuroraErrorInfo::new(
                1200012,
                "get rule form create json error".to_string(),
                "获取规则 FROM-CREATE-JSON 错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryRuleListPagingError(data, _param) => AuroraErrorInfo::new(
                1200013,
                "query rule list paging error".to_string(),
                "获取规则分页列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryRuleListError(data, _param) => AuroraErrorInfo::new(
                1200014,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryRuleInputEntryListError(data, _param) => AuroraErrorInfo::new(
                1200015,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryExecuteResultListPagingError(data, _param) => AuroraErrorInfo::new(
                1200016,
                "query execute result list paging error".to_string(),
                "获取数据质量任务结果分页错误".to_string(),
            )
            .new_with_data(data),
            Error::GetDatasourceOptionsError(data, _param) => AuroraErrorInfo::new(
                1200017,
                "get datasource options error".to_string(),
                "获取数据源Options错误".to_string(),
            )
            .new_with_data(data),
            Error::GetDatasourceTablesError(data, _param) => AuroraErrorInfo::new(
                1200018,
                "get datasource tables error".to_string(),
                "获取数据源表列表错误".to_string(),
            )
            .new_with_data(data),
            Error::GetDatasourceTableColumnsError(data, _param) => AuroraErrorInfo::new(
                1200019,
                "get datasource table columns error".to_string(),
                "获取数据源表列名错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupNameExist(data, _param) => AuroraErrorInfo::new(
                130001,
                "this task group name is repeated in a project".to_string(),
                "该任务组名称在一个项目中已经使用".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupSizeError(data, _param) => AuroraErrorInfo::new(
                130002,
                "task group size error".to_string(),
                "任务组大小应该为大于1的整数".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupStatusError(data, _param) => AuroraErrorInfo::new(
                130003,
                "task group status error".to_string(),
                "任务组已经被关闭".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupFull(data, _param) => {
                AuroraErrorInfo::new(130004, "task group is full".to_string(), "任务组已经满了".to_string())
                    .new_with_data(data)
            }
            Error::TaskGroupUsedSizeError(data, _param) => AuroraErrorInfo::new(
                130005,
                "the used size number of task group is dirty".to_string(),
                "任务组使用的容量发生了变化".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupQueueReleaseError(data, _param) => AuroraErrorInfo::new(
                130006,
                "failed to release task group queue".to_string(),
                "任务组资源释放时出现了错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupQueueAwakeError(data, _param) => AuroraErrorInfo::new(
                130007,
                "awake waiting task failed".to_string(),
                "任务组使唤醒等待任务时发生了错误".to_string(),
            )
            .new_with_data(data),
            Error::CreateTaskGroupError(data, _param) => AuroraErrorInfo::new(
                130008,
                "create task group error".to_string(),
                "创建任务组错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateTaskGroupError(data, _param) => AuroraErrorInfo::new(
                130009,
                "update task group list error".to_string(),
                "更新任务组错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskGroupListError(data, _param) => AuroraErrorInfo::new(
                130010,
                "query task group list error".to_string(),
                "查询任务组列表错误".to_string(),
            )
            .new_with_data(data),
            Error::CloseTaskGroupError(data, _param) => AuroraErrorInfo::new(
                130011,
                "close task group error".to_string(),
                "关闭任务组错误".to_string(),
            )
            .new_with_data(data),
            Error::StartTaskGroupError(data, _param) => AuroraErrorInfo::new(
                130012,
                "start task group error".to_string(),
                "启动任务组错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryTaskGroupQueueListError(data, _param) => AuroraErrorInfo::new(
                130013,
                "query task group queue list error".to_string(),
                "查询任务组队列列表错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupCacheStartFailed(data, _param) => AuroraErrorInfo::new(
                130014,
                "cache start failed".to_string(),
                "任务组相关的缓存启动失败".to_string(),
            )
            .new_with_data(data),
            Error::EnvironmentWorkerGroupsIsInvalid(data, _param) => AuroraErrorInfo::new(
                130015,
                "environment worker groups is invalid format".to_string(),
                "环境关联的工作组参数解析错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateEnvironmentWorkerGroupRelationError(data, _param) => AuroraErrorInfo::new(
                130016,
                "update environment worker group relation error".to_string(),
                "更新环境关联的工作组错误".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupQueueAlreadyStart(data, _param) => AuroraErrorInfo::new(
                130017,
                "task group queue already start".to_string(),
                "节点已经获取任务组资源".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupStatusClosed(data, _param) => AuroraErrorInfo::new(
                130018,
                "The task group has been closed.".to_string(),
                "任务组已经被关闭".to_string(),
            )
            .new_with_data(data),
            Error::TaskGroupStatusOpened(data, _param) => AuroraErrorInfo::new(
                130019,
                "The task group has been opened.".to_string(),
                "任务组已经被开启".to_string(),
            )
            .new_with_data(data),
            Error::NotAllowToDisableOwnAccount(data, _param) => AuroraErrorInfo::new(
                130020,
                "Not allow to disable your own account".to_string(),
                "不能停用自己的账号".to_string(),
            )
            .new_with_data(data),
            Error::NotAllowToDeleteDefaultAlarmGroup(data, _param) => AuroraErrorInfo::new(
                130030,
                "Not allow to delete the default alarm group ".to_string(),
                "不能删除默认告警组".to_string(),
            )
            .new_with_data(data),
            Error::TimeZoneIllegal(data, _param) => AuroraErrorInfo::new(
                130031,
                "time zone [{0}] is illegal".to_string(),
                "时区参数 [{0}] 不合法".to_string(),
            )
            .new_with_data(data),
            Error::QueryK8sNamespaceListPagingError(data, _param) => AuroraErrorInfo::new(
                1300001,
                "login user query k8s namespace list paging error".to_string(),
                "分页查询k8s名称空间列表错误".to_string(),
            )
            .new_with_data(data),
            Error::K8sNamespaceExist(data, _param) => AuroraErrorInfo::new(
                1300002,
                "k8s namespace {0} already exists".to_string(),
                "k8s命名空间[{0}]已存在".to_string(),
            )
            .new_with_data(data),
            Error::CreateK8sNamespaceError(data, _param) => AuroraErrorInfo::new(
                1300003,
                "create k8s namespace error".to_string(),
                "创建k8s命名空间错误".to_string(),
            )
            .new_with_data(data),
            Error::UpdateK8sNamespaceError(data, _param) => AuroraErrorInfo::new(
                1300004,
                "update k8s namespace error".to_string(),
                "更新k8s命名空间信息错误".to_string(),
            )
            .new_with_data(data),
            Error::K8sNamespaceNotExist(data, _param) => AuroraErrorInfo::new(
                1300005,
                "k8s namespace {0} not exists".to_string(),
                "命名空间ID[{0}]不存在".to_string(),
            )
            .new_with_data(data),
            Error::K8sClientOpsError(data, _param) => AuroraErrorInfo::new(
                1300006,
                "k8s error with exception {0}".to_string(),
                "k8s操作报错[{0}]".to_string(),
            )
            .new_with_data(data),
            Error::VerifyK8sNamespaceError(data, _param) => AuroraErrorInfo::new(
                1300007,
                "verify k8s and namespace error".to_string(),
                "验证k8s命名空间信息错误".to_string(),
            )
            .new_with_data(data),
            Error::DeleteK8sNamespaceByIdError(data, _param) => AuroraErrorInfo::new(
                1300008,
                "delete k8s namespace by id error".to_string(),
                "删除命名空间错误".to_string(),
            )
            .new_with_data(data),
            Error::VerifyParameterNameFailed(data, _param) => AuroraErrorInfo::new(
                1300009,
                "The file name verify failed".to_string(),
                "文件命名校验失败".to_string(),
            )
            .new_with_data(data),
            Error::StoreOperateCreateError(data, _param) => AuroraErrorInfo::new(
                1300010,
                "create the resource failed".to_string(),
                "存储操作失败".to_string(),
            )
            .new_with_data(data),
            Error::GrantK8sNamespaceError(data, _param) => {
                AuroraErrorInfo::new(1300011, "grant namespace error".to_string(), "授权资源错误".to_string())
                    .new_with_data(data)
            }
            Error::QueryUnauthorizedNamespaceError(data, _param) => AuroraErrorInfo::new(
                1300012,
                "query unauthorized namespace error".to_string(),
                "查询未授权命名空间错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryAuthorizedNamespaceError(data, _param) => AuroraErrorInfo::new(
                1300013,
                "query authorized namespace error".to_string(),
                "查询授权命名空间错误".to_string(),
            )
            .new_with_data(data),
            Error::QueryCanUseK8sClusterError(data, _param) => AuroraErrorInfo::new(
                1300014,
                "login user query can used k8s cluster list error".to_string(),
                "查询可用k8s集群错误".to_string(),
            )
            .new_with_data(data),
            Error::ResourceFullNameTooLongError(data, _param) => AuroraErrorInfo::new(
                1300015,
                "resource's fullname is too long error".to_string(),
                "资源文件名过长".to_string(),
            )
            .new_with_data(data),
            Error::TenantFullNameTooLongError(data, _param) => AuroraErrorInfo::new(
                1300016,
                "tenant's fullname is too long error".to_string(),
                "租户名过长".to_string(),
            )
            .new_with_data(data),
        }
    }
}
