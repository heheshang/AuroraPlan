use std::{collections::HashMap, str::FromStr};

use axum::{
    http::Extensions,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use tracing::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    SUCCESS,
    InternalServerErrorArgs,
    RequestParamsNotValidError, //(10001, "request parameter {0} is not valid", "请求参数[{0}]无效"),
    TaskTimeoutParamsError, //(10002, "task timeout parameter is not valid", "任务超时参数无效"),
    UserNameExist,          //(10003, "user name already exists", "用户名已存在"),
    UserNameNull,           //(10004, "user name is null", "用户名不能为空"),
    HdfsOperationError,     //(10006, "hdfs operation error", "hdfs操作错误"),
    TaskInstanceNotFound,   //(10008, "task instance not found", "任务实例不存在"),
    OsTenantCodeExist, //(10009, "os tenant code {0} already exists", "操作系统租户[{0}]已存在"),
    UserNotExist,      //(10010, "user {0} not exists", "用户[{0}]不存在"),
    AlertGroupNotExist, //(10011, "alarm group not found", "告警组不存在"),
    AlertGroupExist,   //(10012, "alarm group already exists", "告警组名称已存在"),
    UserNamePasswdError, //(10013, "user name or password error", "用户名或密码错误"),
    LoginSessionFailed, //(10014, "create session failed!", "创建session失败"),
    DatasourceExist,   //(10015, "data source name already exists", "数据源名称已存在"),
    DatasourceConnectFailed, //(10016, "data source connection failed", "建立数据源连接失败"),
    TenantNotExist,    //(10017, "tenant not exists", "租户不存在"),
    ProjectNotFound,   //(10018, "project {0} not found ", "项目[{0}]不存在"),
    ProjectAlreadyExists, //(10019, "project {0} already exists", "项目名称[{0}]已存在"),
    TaskInstanceNotExists, //(10020, "task instance {0} does not exist", "任务实例[{0}]不存在"),
    TaskInstanceNotSubWorkflowInstance, //(10021, "task instance {0} is not sub process instance", "任务实例[{0}]不是子流程实例"),
    ScheduleCronNotExists, //(10022, "scheduler crontab {0} does not exist", "调度配置定时表达式[{0}]不存在"),
    ScheduleCronOnlineForbidUpdate, //(10023, "online status does not allow update operations", "调度配置上线状态不允许修改"),
    ScheduleCronCheckFailed, //(10024, "scheduler crontab expression validation failure: {0}", "调度配置定时表达式验证失败: {0}"),
    MasterNotExists,         //(10025, "master does not exist", "无可用master节点"),
    ScheduleStatusUnknown,   //(10026, "unknown status: {0}", "未知状态: {0}"),
    CreateAlertGroupError,   //(10027, "create alert group error", "创建告警组错误"),
    QueryAllAlertgroupError, //(10028, "query all alertgroup error", "查询告警组错误"),
    ListPagingAlertGroupError, //(10029, "list paging alert group error", "分页查询告警组错误"),
    UpdateAlertGroupError,   //(10030, "update alert group error", "更新告警组错误"),
    DeleteAlertGroupError,   //(10031, "delete alert group error", "删除告警组错误"),
    AlertGroupGrantUserError, //(10032, "alert group grant user error", "告警组授权用户错误"),
    CreateDatasourceError,   //(10033, "create datasource error", "创建数据源错误"),
    UpdateDatasourceError,   //(10034, "update datasource error", "更新数据源错误"),
    QueryDatasourceError,    //(10035, "query datasource error", "查询数据源错误"),
    ConnectDatasourceFailure, //(10036, "connect datasource failure", "建立数据源连接失败"),
    ConnectionTestFailure,   //(10037, "connection test failure", "测试数据源连接失败"),
    DeleteDataSourceFailure, //(10038, "delete data source failure", "删除数据源失败"),
    VerifyDatasourceNameFailure, //(10039, "verify datasource name failure", "验证数据源名称失败"),
    UnauthorizedDatasource,  //(10040, "unauthorized datasource", "未经授权的数据源"),
    AuthorizedDataSource,    //(10041, "authorized data source", "授权数据源失败"),
    LoginSuccess,            //(10042, "login success", "登录成功"),
    UserLoginFailure,        //(10043, "user login failure", "用户登录失败"),
    ListWorkersError,        //(10044, "list workers error", "查询worker列表错误"),
    ListMastersError,        //(10045, "list masters error", "查询master列表错误"),
    UpdateProjectError,      //(10046, "update project error", "更新项目信息错误"),
    QueryProjectDetailsByCodeError, //(10047, "query project details by code error", "查询项目详细信息错误"),
    CreateProjectError,             //(10048, "create project error", "创建项目错误"),
    LoginUserQueryProjectListPagingError, //(10049, "login user query project list paging error", "分页查询项目列表错误"),
    DeleteProjectError,                   //(10050, "delete project error", "删除项目错误"),
    QueryUnauthorizedProjectError, //(10051, "query unauthorized project error", "查询未授权项目错误"),
    QueryAuthorizedProject,        //(10052, "query authorized project", "查询授权项目错误"),
    QueryQueueListError,           //(10053, "query queue list error", "查询队列列表错误"),
    CreateResourceError,           //(10054, "create resource error", "创建资源错误"),
    UpdateResourceError,           //(10055, "update resource error", "更新资源错误"),
    QueryResourcesListError,       //(10056, "query resources list error", "查询资源列表错误"),
    QueryResourcesListPaging,      //(10057, "query resources list paging", "分页查询资源列表错误"),
    DeleteResourceError,           //(10058, "delete resource error", "删除资源错误"),
    VerifyResourceByNameAndTypeError, //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
    ViewResourceFileOnLineError, //(10060, "view resource file online error", "查看资源文件错误"),
    CreateResourceFileOnLineError, //(10061, "create resource file online error", "创建资源文件错误"),
    ResourceFileIsEmpty,           //(10062, "resource file is empty", "资源文件内容不能为空"),
    EditResourceFileOnLineError,   //(10063, "edit resource file online error", "更新资源文件错误"),
    DownloadResourceFileError,     //(10064, "download resource file error", "下载资源文件错误"),
    CreateUdfFunctionError,        //(10065, "create udf function error", "创建UDF函数错误"),
    ViewUdfFunctionError,          //(10066, "view udf function error", "查询UDF函数错误"),
    UpdateUdfFunctionError,        //(10067, "update udf function error", "更新UDF函数错误"),
    QueryUdfFunctionListPagingError, //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
    QueryDatasourceByTypeError, //(10069, "query datasource by type error", "查询数据源信息错误"),
    VerifyUdfFunctionNameError, //(10070, "verify udf function name error", "UDF函数名称验证错误"),
    DeleteUdfFunctionError,     //(10071, "delete udf function error", "删除UDF函数错误"),
    AuthorizedFileResourceError, //(10072, "authorized file resource error", "授权资源文件错误"),
    AuthorizeResourceTree, //(10073, "authorize resource tree display error", "授权资源目录树错误"),
    UnauthorizedUdfFunctionError, //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
    AuthorizedUdfFunctionError,   //(10075, "authorized udf function error", "授权UDF函数错误"),
    CreateScheduleError,          //(10076, "create schedule error", "创建调度配置错误"),
    UpdateScheduleError,          //(10077, "update schedule error", "更新调度配置错误"),
    PublishScheduleOnlineError,   //(10078, "publish schedule online error", "上线调度配置错误"),
    OfflineScheduleError,         //(10079, "offline schedule error", "下线调度配置错误"),
    QueryScheduleListPagingError, //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
    QueryScheduleListError,       //(10081, "query schedule list error", "查询调度配置列表错误"),
    QueryTaskListPagingError,     //(10082, "query task list paging error", "分页查询任务列表错误"),
    QueryTaskRecordListPagingError, //(10083, "query task record list paging error", "分页查询任务记录错误"),
    CreateTenantError,              //(10084, "create tenant error", "创建租户错误"),
    QueryTenantListPagingError, //(10085, "query tenant list paging error", "分页查询租户列表错误"),
    QueryTenantListError,       //(10086, "query tenant list error", "查询租户列表错误"),
    UpdateTenantError,          //(10087, "update tenant error", "更新租户错误"),
    DeleteTenantByIdError,      //(10088, "delete tenant by id error", "删除租户错误"),
    VerifyOsTenantCodeError,    //(10089, "verify os tenant code error", "操作系统租户验证错误"),
    CreateUserError,            //(10090, "create user error", "创建用户错误"),
    QueryUserListPagingError,   //(10091, "query user list paging error", "分页查询用户列表错误"),
    UpdateUserError,            //(10092, "update user error", "更新用户错误"),
    DeleteUserByIdError,        //(10093, "delete user by id error", "删除用户错误"),
    GrantProjectError,          //(10094, "grant project error", "授权项目错误"),
    GrantResourceError,         //(10095, "grant resource error", "授权资源错误"),
    GrantUdfFunctionError,      //(10096, "grant udf function error", "授权UDF函数错误"),
    GrantDatasourceError,       //(10097, "grant datasource error", "授权数据源错误"),
    GetUserInfoError,           //(10098, "get user info error", "获取用户信息错误"),
    UserListError,              //(10099, "user list error", "查询用户列表错误"),
    VerifyUsernameError,        //(10100, "verify username error", "用户名验证错误"),
    UnauthorizedUserError,      //(10101, "unauthorized user error", "查询未授权用户错误"),
    AuthorizedUserError,        //(10102, "authorized user error", "查询授权用户错误"),
    QueryTaskInstanceLogError,  //(10103, "view task instance log error", "查询任务实例日志错误"),
    DownloadTaskInstanceLogFileError, //(10104, "download task instance log file error", "下载任务日志文件错误"),
    CreateProcessDefinitionError, //(10105, "create process definition error", "创建工作流错误"),
    VerifyProcessDefinitionNameUniqueError, //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
    UpdateProcessDefinitionError, //(10107, "update process definition error", "更新工作流定义错误"),
    ReleaseProcessDefinitionError, //(10108, "release process definition error", "上线工作流错误"),
    QueryDetailOfProcessDefinitionError, //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
    QueryProcessDefinitionList, //(10110, "query process definition list", "查询工作流列表错误"),
    EncapsulationTreeviewStructureError, //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
    GetTasksListByProcessDefinitionIdError, //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
    QueryProcessInstanceListPagingError, //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
    QueryTaskListByProcessInstanceIdError, //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
    UpdateProcessInstanceError, //(10115, "update process instance error", "更新工作流实例错误"),
    QueryProcessInstanceByIdError, //(10116, "query process instance by id error", "查询工作流实例错误"),
    DeleteProcessInstanceByIdError, //(10117, "delete process instance by id error", "删除工作流实例错误"),
    QuerySubProcessInstanceDetailInfoByTaskIdError, //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
    QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError, //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
    QueryProcessInstanceAllVariablesError, //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
    EncapsulationProcessInstanceGanttStructureError, //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
    QueryProcessDefinitionListPagingError, //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
    SignOutError,                          //(10123, "sign out error", "退出错误"),
    OsTenantCodeHasAlreadyExists, //(10124, "os tenant code has already exists", "操作系统租户已存在"),
    IpIsEmpty,                    //(10125, "ip is empty", "IP地址不能为空"),
    ScheduleCronReleaseNeedNotChange, //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
    CreateQueueError,                 //(10127, "create queue error", "创建队列错误"),
    QueueNotExist,                    //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
    QueueValueExist, //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
    QueueNameExist,  //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
    UpdateQueueError, //(10131, "update queue error", "更新队列信息错误"),
    NeedNotUpdateQueue, //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
    VerifyQueueError,   //(10133, "verify queue error", "验证队列信息错误"),
    NameNull,           //(10134, "name must be not null", "名称不能为空"),
    NameExist,          //(10135, "name {0} already exists", "名称[{0}]已存在"),
    SaveError,          //(10136, "save error", "保存错误"),
    DeleteProjectErrorDefinesNotNull, //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
    BatchDeleteProcessInstanceByIdsError, //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
    PreviewScheduleError,                 //(10139, "preview schedule error", "预览调度配置错误"),
    ParseToCronExpressionError, //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
    ScheduleStartTimeEndTimeSame, //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
    DeleteTenantByIdFail, //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
    DeleteTenantByIdFailDefines, //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
    DeleteTenantByIdFailUsers, //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
    DeleteWorkerGroupByIdFail, //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
    QueryWorkerGroupFail,      //(10146, "query worker group fail ", "查询worker分组失败"),
    DeleteWorkerGroupFail,     //(10147, "delete worker group fail ", "删除worker分组失败"),
    UserDisabled,              //(10148, "The current user is disabled", "当前用户已停用"),
    CopyProcessDefinitionError, //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
    MoveProcessDefinitionError, //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
    SwitchProcessDefinitionVersionError, //(10151, "Switch process definition version error", "切换工作流版本出错"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionError, //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError, //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
    QueryProcessDefinitionVersionsError, //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
    DeleteProcessDefinitionVersionError, //(10156, "delete process definition version error", "删除工作流历史版本出错"),

    QueryUserCreatedProjectError, //(10157, "query user created project error error", "查询用户创建的项目错误"),
    ProcessDefinitionCodesIsEmpty, //(10158, "process definition codes is empty", "工作流CODES不能为空"),
    BatchCopyProcessDefinitionError, //(10159, "batch copy process definition error", "复制工作流错误"),
    BatchMoveProcessDefinitionError, //(10160, "batch move process definition error", "移动工作流错误"),
    QueryWorkflowLineageError,       //(10161, "query workflow lineage error", "查询血缘失败"),
    QueryAuthorizedAndUserCreatedProjectError, //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
    DeleteProcessDefinitionByCodeFail, //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
    CheckOsTenantCodeError, //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
    ForceTaskSuccessError,  //(10165, "force task success error", "强制成功任务实例错误"),
    TaskInstanceStateOperationError, //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
    DatasourceTypeNotExist,          //(10167, "data source type not exist", "数据源类型不存在"),
    ProcessDefinitionNameExist, //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
    DatasourceDbTypeIllegal,    //(10169, "datasource type illegal", "数据源类型参数不合法"),
    DatasourcePortIllegal,      //(10170, "datasource port illegal", "数据源端口参数不合法"),
    DatasourceOtherParamsIllegal, //(10171, "datasource other params illegal", "数据源其他参数不合法"),
    DatasourceNameIllegal,        //(10172, "datasource name illegal", "数据源名称不合法"),
    DatasourceHostIllegal,        //(10173, "datasource host illegal", "数据源HOST不合法"),
    DeleteWorkerGroupNotExist, //(10174, "delete worker group not exist ", "删除worker分组不存在"),
    CreateWorkerGroupForbiddenInDocker, //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
    DeleteWorkerGroupForbiddenInDocker, //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
    WorkerAddressInvalid, //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
    QueryWorkerAddressListFail, //(10178, "query worker address list fail ", "查询worker地址列表失败"),
    TransformProjectOwnership, //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
    QueryAlertGroupError,      //(10180, "query alert group error", "查询告警组错误"),
    CurrentLoginUserTenantNotExist, //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
    RevokeProjectError,             //(10182, "revoke project error", "撤销项目授权错误"),
    QueryAuthorizedUser, //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
    ProjectNotExist, //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
    TaskInstanceHostIsNull, //(10191, "task instance host is null", "任务实例host为空"),
    QueryExecutingWorkflowError, //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

    UdfFunctionNotExist, //(20001, "UDF function not found", "UDF函数不存在"),
    UdfFunctionExists,   //(20002, "UDF function already exists", "UDF函数已存在"),
    ResourceNotExist,    //(20004, "resource not exist", "资源不存在"),
    ResourceExist,       //(20005, "resource already exists", "资源已存在"),
    ResourceSuffixNotSupportView, //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
    ResourceSizeExceedLimit, //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
    ResourceSuffixForbidChange, //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
    UdfResourceSuffixNotJar, //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
    HdfsCopyFail,            //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
    ResourceFileExist, //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
    ResourceFileNotExist, //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
    UdfResourceIsBound, //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
    ResourceIsUsed, //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
    ParentResourceNotExist, //(20015, "parent resource not exist", "父资源文件不存在"),
    ResourceNotExistOrNoPermission, //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
    ResourceIsAuthorized, //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

    UserNoOperationPerm, //(30001, "user has no operation privilege", "当前用户没有操作权限"),
    UserNoOperationProjectPerm, //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

    ProcessInstanceNotExist, //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
    ProcessInstanceExist, //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
    ProcessDefineNotExist, //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
    ProcessDefineNotRelease, //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
    SubProcessDefineNotRelease, //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
    ProcessInstanceAlreadyChanged, //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
    ProcessInstanceStateOperationError, //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
    SubProcessInstanceNotExist, //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
    ProcessDefineNotAllowedEdit, //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
    ProcessInstanceExecutingCommand, //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
    ProcessInstanceNotSubProcessInstance, //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
    TaskInstanceStateCountError, //(50011, "task instance state count error", "查询各状态任务实例数错误"),
    CountProcessInstanceStateError, //(50012, "count process instance state error", "查询各状态流程实例数错误"),
    CountProcessDefinitionUserError, //(50013, "count process definition user error", "查询各用户流程定义数错误"),
    StartProcessInstanceError, //(50014, "start process instance error", "运行工作流实例错误"),
    BatchStartProcessInstanceError, //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
    ProcessInstanceError, //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
    ExecuteProcessInstanceError, //(50015, "execute process instance error", "操作工作流实例错误"),
    CheckProcessDefinitionError, //(50016, "check process definition error", "工作流定义错误"),
    QueryRecipientsAndCopyersByProcessDefinitionError, //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误"),
    DataIsNotValid,                 //(50017, "data {0} not valid", "数据[{0}]无效"),
    DataIsNull,                     //(50018, "data {0} is null", "数据[{0}]不能为空"),
    ProcessNodeHasCycle,            //(50019, "process node has cycle", "流程节点间存在循环依赖"),
    ProcessNodeSParameterInvalid, //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效"),
    ProcessDefineStateOnline, //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线"),
    DeleteProcessDefineByCodeError, //(50022, "delete process definition by code error", "删除工作流定义错误"),
    ScheduleCronStateOnline, //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线"),
    DeleteScheduleCronByIdError, //(50024, "delete schedule by id error", "删除调度配置错误"),
    BatchDeleteProcessDefineError, //(50025, "batch delete process definition error", "批量删除工作流定义错误"),
    BatchDeleteProcessDefineByCodesError, //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误"),
    DeleteProcessDefineByCodesError, //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误"),
    TenantNotSuitable, //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户"),
    ExportProcessDefineByIdError, //(50028, "export process definition by id error", "导出工作流定义错误"),
    BatchExportProcessDefineByIdsError, //(50028, "batch export process definition by ids error", "批量导出工作流定义错误"),
    ImportProcessDefineError, //(50029, "import process definition error", "导入工作流定义错误"),
    TaskDefineNotExist, //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在"),
    CreateProcessTaskRelationError, //(50032, "create process task relation error", "创建工作流任务关系错误"),
    ProcessTaskRelationNotExist, //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在"),
    ProcessTaskRelationExist, //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]"),
    ProcessDagIsEmpty,        //(50035, "process dag is empty", "工作流dag是空"),
    CheckProcessTaskRelationError, //(50036, "check process task relation error", "工作流任务关系参数错误"),
    CreateTaskDefinitionError,     //(50037, "create task definition error", "创建任务错误"),
    UpdateTaskDefinitionError,     //(50038, "update task definition error", "更新任务定义错误"),
    QueryTaskDefinitionVersionsError, //(50039, "query task definition versions error", "查询任务历史版本信息出错"),
    SwitchTaskDefinitionVersionError, //(50040, "Switch task definition version error", "切换任务版本出错"),
    DeleteTaskDefinitionVersionError, //(50041, "delete task definition version error", "删除任务历史版本出错"),
    DeleteTaskDefineByCodeError, //(50042, "delete task definition by code error", "删除任务定义错误"),
    QueryDetailOfTaskDefinitionError, //(50043, "query detail of task definition error", "查询任务详细信息错误"),
    QueryTaskDefinitionListPagingError, //(50044, "query task definition list paging error", "分页查询任务定义列表错误"),
    TaskDefinitionNameExisted, //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在"),
    ReleaseTaskDefinitionError, //(50046, "release task definition error", "上线任务错误"),
    MoveProcessTaskRelationError, //(50047, "move process task relation error", "移动任务到其他工作流错误"),
    DeleteTaskProcessRelationError, //(50048, "delete process task relation error", "删除工作流任务关系错误"),
    QueryTaskProcessRelationError, //(50049, "query process task relation error", "查询工作流任务关系错误"),
    TaskDefineStateOnline, //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线"),
    TaskHasDownstream, //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖"),
    TaskHasUpstream,   //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖"),
    MainTableUsingVersion, //(50053, "the version that the master table is using", "主表正在使用该版本"),
    ProjectProcessNotMatch, //(50054, "the project and the process is not match", "项目和工作流不匹配"),
    DeleteEdgeError,        //(50055, "delete edge error", "删除工作流任务连接线错误"),
    NotSupportUpdateTaskDefinition, //(50056, "task state does not support modification", "当前任务不支持修改"),
    NotSupportCopyTaskType, //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]"),
    HdfsNotStartup,         //(60001, "hdfs not startup", "hdfs未启用"),
    StorageNotStartup,      //(60002, "storage not startup", "存储未启用"),
    S3CannotRename,         //(60003, "directory cannot be renamed", "S3无法重命名文件夹"),
    /**
     * for monitor
     */
    QueryDatabaseStateError, //(70001, "query database state error", "查询数据库状态错误"),

    CreateAccessTokenError, //(70010, "create access token error", "创建访问token错误"),
    GenerateTokenError,     //(70011, "generate token error", "生成token错误"),
    QueryAccesstokenListPagingError, //(70012, "query access token list paging error", "分页查询访问token列表错误"),
    UpdateAccessTokenError,          //(70013, "update access token error", "更新访问token错误"),
    DeleteAccessTokenError,          //(70014, "delete access token error", "删除访问token错误"),
    AccessTokenNotExist,             //(70015, "access token not exist", "访问token不存在"),
    QueryAccesstokenByUserError, //(70016, "query access token by user error", "查询访问指定用户的token错误"),

    CommandStateCountError, //(80001, "task instance state count error", "查询各状态任务实例数错误"),
    NegativeSizeNumberError, //(80002, "query size number error", "查询size错误"),
    StartTimeBiggerThanEndTimeError, //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误"),
    QueueCountError,                 //(90001, "queue count error", "查询队列数据错误"),

    KerberosStartupState, //(100001, "get kerberos startup state error", "获取kerberos启动状态错误"),

    // audit log
    QueryAuditLogListPaging, //(10057, "query resources list paging", "分页查询资源列表错误"),

    //plugin
    PluginNotAUiComponent, //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件"),
    QueryPluginsResultIsNull, //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功"),
    QueryPluginsError,        //(110003, "query plugins error", "查询插件错误"),
    QueryPluginDetailResultIsNull, //(110004, "query plugin detail result is null", "查询插件详情结果为空"),

    UpdateAlertPluginInstanceError, //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误"),
    DeleteAlertPluginInstanceError, //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误"),
    GetAlertPluginInstanceError, //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误"),
    CreateAlertPluginInstanceError, //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误"),
    QueryAllAlertPluginInstanceError, //(110009, "query all alert plugin instance error", "查询所有告警实例失败"),
    PluginInstanceAlreadyExit, //(110010, "plugin instance already exit", "该告警插件实例已存在"),
    ListPagingAlertPluginInstanceError, //(110011, "query plugin instance page error", "分页查询告警实例失败"),
    DeleteAlertPluginInstanceErrorHasAlertGroupAssociated, //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组"),
    ProcessDefinitionVersionIsUsed, //(110013, "this process definition version is used", "此工作流定义版本被使用"),

    CreateEnvironmentError, //(120001, "create environment error", "创建环境失败"),
    EnvironmentNameExists, //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在"),
    EnvironmentNameIsNull, //(120003, "this environment name shouldn't be empty.", "环境名称不能为空"),
    EnvironmentConfigIsNull, //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空"),
    UpdateEnvironmentError, //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败"),
    DeleteEnvironmentError, //(120006, "delete environment error", "删除环境信息失败"),
    DeleteEnvironmentRelatedTaskExists, //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息"),
    QueryEnvironmentByNameError, //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在"),
    QueryEnvironmentByCodeError, //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在"),
    QueryEnvironmentError, //(1200010, "login user query environment error", "分页查询环境列表错误"),
    VerifyEnvironmentError, //(1200011, "verify environment error", "验证环境信息错误"),
    GetRuleFormCreateJsonError, //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误"),
    QueryRuleListPagingError,   //(1200013, "query rule list paging error", "获取规则分页列表错误"),
    QueryRuleListError,         //(1200014, "query rule list error", "获取规则列表错误"),
    QueryRuleInputEntryListError, //(1200015, "query rule list error", "获取规则列表错误"),
    QueryExecuteResultListPagingError, //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误"),
    GetDatasourceOptionsError, //(1200017, "get datasource options error", "获取数据源Options错误"),
    GetDatasourceTablesError,  //(1200018, "get datasource tables error", "获取数据源表列表错误"),
    GetDatasourceTableColumnsError, //(1200019, "get datasource table columns error", "获取数据源表列名错误"),
    TaskGroupNameExist, //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用"),
    TaskGroupSizeError, //(130002, "task group size error", "任务组大小应该为大于1的整数"),
    TaskGroupStatusError, //(130003, "task group status error", "任务组已经被关闭"),
    TaskGroupFull,      //(130004, "task group is full", "任务组已经满了"),
    TaskGroupUsedSizeError, //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化"),
    TaskGroupQueueReleaseError, //(130006, "failed to release task group queue", "任务组资源释放时出现了错误"),
    TaskGroupQueueAwakeError, //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误"),
    CreateTaskGroupError,     //(130008, "create task group error", "创建任务组错误"),
    UpdateTaskGroupError,     //(130009, "update task group list error", "更新任务组错误"),
    QueryTaskGroupListError,  //(130010, "query task group list error", "查询任务组列表错误"),
    CloseTaskGroupError,      //(130011, "close task group error", "关闭任务组错误"),
    StartTaskGroupError,      //(130012, "start task group error", "启动任务组错误"),
    QueryTaskGroupQueueListError, //(130013, "query task group queue list error", "查询任务组队列列表错误"),
    TaskGroupCacheStartFailed,    //(130014, "cache start failed", "任务组相关的缓存启动失败"),
    EnvironmentWorkerGroupsIsInvalid, //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误"),
    UpdateEnvironmentWorkerGroupRelationError, //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中"),
    TaskGroupQueueAlreadyStart, //(130017, "task group queue already start", "节点已经获取任务组资源"),
    TaskGroupStatusClosed,      //(130018, "The task group has been closed.", "任务组已经被关闭"),
    TaskGroupStatusOpened,      //(130019, "The task group has been opened.", "任务组已经被开启"),
    NotAllowToDisableOwnAccount, //(130020, "Not allow to disable your own account", "不能停用自己的账号"),
    NotAllowToDeleteDefaultAlarmGroup, //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组"),
    TimeZoneIllegal, //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法"),

    QueryK8sNamespaceListPagingError, //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误"),
    K8sNamespaceExist, //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在"),
    CreateK8sNamespaceError, //(1300003, "create k8s namespace error", "创建k8s命名空间错误"),
    UpdateK8sNamespaceError, //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误"),
    K8sNamespaceNotExist, //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在"),
    K8sClientOpsError, //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]"),
    VerifyK8sNamespaceError, //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误"),
    DeleteK8sNamespaceByIdError, //(1300008, "delete k8s namespace by id error", "删除命名空间错误"),
    VerifyParameterNameFailed,   //(1300009, "The file name verify failed", "文件命名校验失败"),
    StoreOperateCreateError,     //(1300010, "create the resource failed", "存储操作失败"),
    GrantK8sNamespaceError,      //(1300011, "grant namespace error", "授权资源错误"),
    QueryUnauthorizedNamespaceError, //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误"),
    QueryAuthorizedNamespaceError, //(1300013, "query authorized namespace error", "查询授权命名空间错误"),
    QueryCanUseK8sClusterError, //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误"),
    ResourceFullNameTooLongError, //(1300015, "resource's fullname is too long error", "资源文件名过长"),
    TenantFullNameTooLongError,   //(1300016, "tenant's fullname is too long error", "租户名过长");
}
impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        error!("{:<12} - model::Error {val:?}", "FROM_JSON");
        Self::InternalServerErrorArgs
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
            Error::SUCCESS => tonic::Status::new(tonic::Code::Ok, "success"),
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
        Self::SUCCESS
    }
}

impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        if value.code() == tonic::Code::Ok {
            return Error::SUCCESS;
        }
        if value.code() == tonic::Code::Internal {
            return Error::InternalServerErrorArgs;
        }
        let code = value.code();
        if code == tonic::Code::Unknown {
            let message = value.message().split('~');
            let mut map = HashMap::new();
            for m in message {
                let kv = m.split(':').collect::<Vec<&str>>();
                if kv.len() == 2 {
                    map.insert(kv[0].to_string(), kv[1].to_string());
                }
            }

            let error_code = String::from("50000");
            let cn_msg = String::from("未知错误");
            let en_msg = String::from("unknown error");
            let error_code = map.get("code").unwrap_or(&error_code);
            let cn_msg = map.get("cn_msg").unwrap_or(&cn_msg);
            let en_msg = map.get("en_msg").unwrap_or(&en_msg);
            let error_code: i32 = error_code.parse().unwrap();
            let error = AuroraErrorInfo {
                code: error_code,
                cn_msg: cn_msg.to_string(),
                en_msg: en_msg.to_string(),
            };
            let error: Error = error.into();
            error
        } else {
            Error::InternalServerErrorArgs
        }
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::SUCCESS => {
                let ss: AuroraErrorInfo = Error::SUCCESS.into();
                write!(f, "{}", ss)
            }
            Error::InternalServerErrorArgs => {
                let ss: AuroraErrorInfo = Error::InternalServerErrorArgs.into();
                write!(f, "{}", ss)
            }
            Error::RequestParamsNotValidError => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError.into();
                write!(f, "{}", ss)
            }

            Error::TaskTimeoutParamsError => {
                let ss: AuroraErrorInfo = Error::TaskTimeoutParamsError.into();
                write!(f, "{}", ss)
            }
            Error::UserNameExist => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError.into();
                write!(f, "{}", ss)
            }
            Error::UserNameNull => {
                let ss: AuroraErrorInfo = Error::UserNameNull.into();
                write!(f, "{}", ss)
            }
            Error::HdfsOperationError => {
                let ss: AuroraErrorInfo = Error::HdfsOperationError.into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotFound => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotFound.into();
                write!(f, "{}", ss)
            }
            Error::OsTenantCodeExist => {
                let ss: AuroraErrorInfo = Error::OsTenantCodeExist.into();
                write!(f, "{}", ss)
            }
            Error::UserNotExist => {
                let ss: AuroraErrorInfo = Error::UserNotExist.into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupNotExist => {
                let ss: AuroraErrorInfo = Error::AlertGroupNotExist.into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupExist => {
                let ss: AuroraErrorInfo = Error::AlertGroupExist.into();
                write!(f, "{}", ss)
            }
            Error::UserNamePasswdError => {
                let ss: AuroraErrorInfo = Error::UserNamePasswdError.into();
                write!(f, "{}", ss)
            }
            Error::LoginSessionFailed => {
                let ss: AuroraErrorInfo = Error::LoginSessionFailed.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceExist => {
                let ss: AuroraErrorInfo = Error::DatasourceExist.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceConnectFailed => {
                let ss: AuroraErrorInfo = Error::DatasourceConnectFailed.into();
                write!(f, "{}", ss)
            }
            Error::TenantNotExist => {
                let ss: AuroraErrorInfo = Error::TenantNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ProjectNotFound => {
                let ss: AuroraErrorInfo = Error::ProjectNotFound.into();
                write!(f, "{}", ss)
            }
            Error::ProjectAlreadyExists => {
                let ss: AuroraErrorInfo = Error::ProjectAlreadyExists.into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotExists => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotExists.into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceNotSubWorkflowInstance => {
                let ss: AuroraErrorInfo = Error::TaskInstanceNotSubWorkflowInstance.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronNotExists => {
                let ss: AuroraErrorInfo = Error::ScheduleCronNotExists.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronOnlineForbidUpdate => {
                let ss: AuroraErrorInfo = Error::ScheduleCronOnlineForbidUpdate.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronCheckFailed => {
                let ss: AuroraErrorInfo = Error::ScheduleCronCheckFailed.into();
                write!(f, "{}", ss)
            }
            Error::MasterNotExists => {
                let ss: AuroraErrorInfo = Error::MasterNotExists.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleStatusUnknown => {
                let ss: AuroraErrorInfo = Error::ScheduleStatusUnknown.into();
                write!(f, "{}", ss)
            }
            Error::CreateAlertGroupError => {
                let ss: AuroraErrorInfo = Error::CreateAlertGroupError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAllAlertgroupError => {
                let ss: AuroraErrorInfo = Error::QueryAllAlertgroupError.into();
                write!(f, "{}", ss)
            }
            Error::ListPagingAlertGroupError => {
                let ss: AuroraErrorInfo = Error::ListPagingAlertGroupError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateAlertGroupError => {
                let ss: AuroraErrorInfo = Error::UpdateAlertGroupError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertGroupError => {
                let ss: AuroraErrorInfo = Error::DeleteAlertGroupError.into();
                write!(f, "{}", ss)
            }
            Error::AlertGroupGrantUserError => {
                let ss: AuroraErrorInfo = Error::AlertGroupGrantUserError.into();
                write!(f, "{}", ss)
            }
            Error::CreateDatasourceError => {
                let ss: AuroraErrorInfo = Error::CreateDatasourceError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateDatasourceError => {
                let ss: AuroraErrorInfo = Error::UpdateDatasourceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryDatasourceError => {
                let ss: AuroraErrorInfo = Error::QueryDatasourceError.into();
                write!(f, "{}", ss)
            }
            Error::ConnectDatasourceFailure => {
                let ss: AuroraErrorInfo = Error::ConnectDatasourceFailure.into();
                write!(f, "{}", ss)
            }
            Error::ConnectionTestFailure => {
                let ss: AuroraErrorInfo = Error::ConnectionTestFailure.into();
                write!(f, "{}", ss)
            }
            Error::DeleteDataSourceFailure => {
                let ss: AuroraErrorInfo = Error::DeleteDataSourceFailure.into();
                write!(f, "{}", ss)
            }
            Error::VerifyDatasourceNameFailure => {
                let ss: AuroraErrorInfo = Error::VerifyDatasourceNameFailure.into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedDatasource => {
                let ss: AuroraErrorInfo = Error::UnauthorizedDatasource.into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedDataSource => {
                let ss: AuroraErrorInfo = Error::AuthorizedDataSource.into();
                write!(f, "{}", ss)
            }
            Error::LoginSuccess => {
                let ss: AuroraErrorInfo = Error::LoginSuccess.into();
                write!(f, "{}", ss)
            }
            Error::UserLoginFailure => {
                let ss: AuroraErrorInfo = Error::UserLoginFailure.into();
                write!(f, "{}", ss)
            }
            Error::ListWorkersError => {
                let ss: AuroraErrorInfo = Error::ListWorkersError.into();
                write!(f, "{}", ss)
            }
            Error::ListMastersError => {
                let ss: AuroraErrorInfo = Error::ListMastersError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateProjectError => {
                let ss: AuroraErrorInfo = Error::UpdateProjectError.into();
                write!(f, "{}", ss)
            }
            Error::QueryProjectDetailsByCodeError => {
                let ss: AuroraErrorInfo = Error::QueryProjectDetailsByCodeError.into();
                write!(f, "{}", ss)
            }
            Error::CreateProjectError => {
                let ss: AuroraErrorInfo = Error::CreateProjectError.into();
                write!(f, "{}", ss)
            }
            Error::LoginUserQueryProjectListPagingError => {
                let ss: AuroraErrorInfo = Error::LoginUserQueryProjectListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProjectError => {
                let ss: AuroraErrorInfo = Error::DeleteProjectError.into();
                write!(f, "{}", ss)
            }
            Error::QueryUnauthorizedProjectError => {
                let ss: AuroraErrorInfo = Error::QueryUnauthorizedProjectError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedProject => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedProject.into();
                write!(f, "{}", ss)
            }
            Error::QueryQueueListError => {
                let ss: AuroraErrorInfo = Error::QueryQueueListError.into();
                write!(f, "{}", ss)
            }
            Error::CreateResourceError => {
                let ss: AuroraErrorInfo = Error::CreateResourceError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateResourceError => {
                let ss: AuroraErrorInfo = Error::UpdateResourceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryResourcesListError => {
                let ss: AuroraErrorInfo = Error::QueryResourcesListError.into();
                write!(f, "{}", ss)
            }
            Error::QueryResourcesListPaging => {
                let ss: AuroraErrorInfo = Error::QueryResourcesListPaging.into();
                write!(f, "{}", ss)
            }
            Error::DeleteResourceError => {
                let ss: AuroraErrorInfo = Error::DeleteResourceError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyResourceByNameAndTypeError => {
                let ss: AuroraErrorInfo = Error::VerifyResourceByNameAndTypeError.into();
                write!(f, "{}", ss)
            }
            Error::ViewResourceFileOnLineError => {
                let ss: AuroraErrorInfo = Error::ViewResourceFileOnLineError.into();
                write!(f, "{}", ss)
            }
            Error::CreateResourceFileOnLineError => {
                let ss: AuroraErrorInfo = Error::CreateResourceFileOnLineError.into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileIsEmpty => {
                let ss: AuroraErrorInfo = Error::ResourceFileIsEmpty.into();
                write!(f, "{}", ss)
            }
            Error::EditResourceFileOnLineError => {
                let ss: AuroraErrorInfo = Error::EditResourceFileOnLineError.into();
                write!(f, "{}", ss)
            }
            Error::DownloadResourceFileError => {
                let ss: AuroraErrorInfo = Error::DownloadResourceFileError.into();
                write!(f, "{}", ss)
            }
            Error::CreateUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::CreateUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::ViewUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::ViewUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::UpdateUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryUdfFunctionListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryUdfFunctionListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::QueryDatasourceByTypeError => {
                let ss: AuroraErrorInfo = Error::QueryDatasourceByTypeError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyUdfFunctionNameError => {
                let ss: AuroraErrorInfo = Error::VerifyUdfFunctionNameError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::DeleteUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedFileResourceError => {
                let ss: AuroraErrorInfo = Error::AuthorizedFileResourceError.into();
                write!(f, "{}", ss)
            }
            Error::AuthorizeResourceTree => {
                let ss: AuroraErrorInfo = Error::AuthorizeResourceTree.into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::UnauthorizedUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::AuthorizedUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::CreateScheduleError => {
                let ss: AuroraErrorInfo = Error::CreateScheduleError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateScheduleError => {
                let ss: AuroraErrorInfo = Error::UpdateScheduleError.into();
                write!(f, "{}", ss)
            }
            Error::PublishScheduleOnlineError => {
                let ss: AuroraErrorInfo = Error::PublishScheduleOnlineError.into();
                write!(f, "{}", ss)
            }
            Error::OfflineScheduleError => {
                let ss: AuroraErrorInfo = Error::OfflineScheduleError.into();
                write!(f, "{}", ss)
            }
            Error::QueryScheduleListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryScheduleListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::QueryScheduleListError => {
                let ss: AuroraErrorInfo = Error::QueryScheduleListError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryTaskListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskRecordListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryTaskRecordListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::CreateTenantError => {
                let ss: AuroraErrorInfo = Error::CreateTenantError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTenantListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryTenantListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTenantListError => {
                let ss: AuroraErrorInfo = Error::QueryTenantListError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateTenantError => {
                let ss: AuroraErrorInfo = Error::UpdateTenantError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdError => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyOsTenantCodeError => {
                let ss: AuroraErrorInfo = Error::VerifyOsTenantCodeError.into();
                write!(f, "{}", ss)
            }
            Error::CreateUserError => {
                let ss: AuroraErrorInfo = Error::CreateUserError.into();
                write!(f, "{}", ss)
            }
            Error::QueryUserListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryUserListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateUserError => {
                let ss: AuroraErrorInfo = Error::UpdateUserError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteUserByIdError => {
                let ss: AuroraErrorInfo = Error::DeleteUserByIdError.into();
                write!(f, "{}", ss)
            }
            Error::GrantProjectError => {
                let ss: AuroraErrorInfo = Error::GrantProjectError.into();
                write!(f, "{}", ss)
            }
            Error::GrantResourceError => {
                let ss: AuroraErrorInfo = Error::GrantResourceError.into();
                write!(f, "{}", ss)
            }
            Error::GrantUdfFunctionError => {
                let ss: AuroraErrorInfo = Error::GrantUdfFunctionError.into();
                write!(f, "{}", ss)
            }
            Error::GrantDatasourceError => {
                let ss: AuroraErrorInfo = Error::GrantDatasourceError.into();
                write!(f, "{}", ss)
            }
            Error::GetUserInfoError => {
                let ss: AuroraErrorInfo = Error::GetUserInfoError.into();
                write!(f, "{}", ss)
            }
            Error::UserListError => {
                let ss: AuroraErrorInfo = Error::UserListError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyUsernameError => {
                let ss: AuroraErrorInfo = Error::VerifyUsernameError.into();
                write!(f, "{}", ss)
            }
            Error::UnauthorizedUserError => {
                let ss: AuroraErrorInfo = Error::UnauthorizedUserError.into();
                write!(f, "{}", ss)
            }
            Error::AuthorizedUserError => {
                let ss: AuroraErrorInfo = Error::AuthorizedUserError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskInstanceLogError => {
                let ss: AuroraErrorInfo = Error::QueryTaskInstanceLogError.into();
                write!(f, "{}", ss)
            }
            Error::DownloadTaskInstanceLogFileError => {
                let ss: AuroraErrorInfo = Error::DownloadTaskInstanceLogFileError.into();
                write!(f, "{}", ss)
            }
            Error::CreateProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::CreateProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyProcessDefinitionNameUniqueError => {
                let ss: AuroraErrorInfo = Error::VerifyProcessDefinitionNameUniqueError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::UpdateProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::ReleaseProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::ReleaseProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryDetailOfProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::QueryDetailOfProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionList => {
                let ss: AuroraErrorInfo = Error::QueryProcessDefinitionList.into();
                write!(f, "{}", ss)
            }
            Error::EncapsulationTreeviewStructureError => {
                let ss: AuroraErrorInfo = Error::EncapsulationTreeviewStructureError.into();
                write!(f, "{}", ss)
            }
            Error::GetTasksListByProcessDefinitionIdError => {
                let ss: AuroraErrorInfo = Error::GetTasksListByProcessDefinitionIdError.into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryProcessInstanceListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskListByProcessInstanceIdError => {
                let ss: AuroraErrorInfo = Error::QueryTaskListByProcessInstanceIdError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateProcessInstanceError => {
                let ss: AuroraErrorInfo = Error::UpdateProcessInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceByIdError => {
                let ss: AuroraErrorInfo = Error::QueryProcessInstanceByIdError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessInstanceByIdError => {
                let ss: AuroraErrorInfo = Error::DeleteProcessInstanceByIdError.into();
                write!(f, "{}", ss)
            }
            Error::QuerySubProcessInstanceDetailInfoByTaskIdError => {
                let ss: AuroraErrorInfo =
                    Error::QuerySubProcessInstanceDetailInfoByTaskIdError.into();
                write!(f, "{}", ss)
            }
            Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError => {
                let ss: AuroraErrorInfo =
                    Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError.into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessInstanceAllVariablesError => {
                let ss: AuroraErrorInfo = Error::QueryProcessInstanceAllVariablesError.into();
                write!(f, "{}", ss)
            }
            Error::EncapsulationProcessInstanceGanttStructureError => {
                let ss: AuroraErrorInfo =
                    Error::EncapsulationProcessInstanceGanttStructureError.into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryProcessDefinitionListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::SignOutError => {
                let ss: AuroraErrorInfo = Error::SignOutError.into();
                write!(f, "{}", ss)
            }
            Error::OsTenantCodeHasAlreadyExists => {
                let ss: AuroraErrorInfo = Error::OsTenantCodeHasAlreadyExists.into();
                write!(f, "{}", ss)
            }
            Error::IpIsEmpty => {
                let ss: AuroraErrorInfo = Error::IpIsEmpty.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronReleaseNeedNotChange => {
                let ss: AuroraErrorInfo = Error::ScheduleCronReleaseNeedNotChange.into();
                write!(f, "{}", ss)
            }
            Error::CreateQueueError => {
                let ss: AuroraErrorInfo = Error::CreateQueueError.into();
                write!(f, "{}", ss)
            }
            Error::QueueNotExist => {
                let ss: AuroraErrorInfo = Error::QueueNotExist.into();
                write!(f, "{}", ss)
            }
            Error::QueueValueExist => {
                let ss: AuroraErrorInfo = Error::QueueValueExist.into();
                write!(f, "{}", ss)
            }
            Error::QueueNameExist => {
                let ss: AuroraErrorInfo = Error::QueueNameExist.into();
                write!(f, "{}", ss)
            }
            Error::UpdateQueueError => {
                let ss: AuroraErrorInfo = Error::UpdateQueueError.into();
                write!(f, "{}", ss)
            }
            Error::NeedNotUpdateQueue => {
                let ss: AuroraErrorInfo = Error::NeedNotUpdateQueue.into();
                write!(f, "{}", ss)
            }
            Error::VerifyQueueError => {
                let ss: AuroraErrorInfo = Error::VerifyQueueError.into();
                write!(f, "{}", ss)
            }
            Error::NameNull => {
                let ss: AuroraErrorInfo = Error::NameNull.into();
                write!(f, "{}", ss)
            }
            Error::NameExist => {
                let ss: AuroraErrorInfo = Error::NameExist.into();
                write!(f, "{}", ss)
            }
            Error::SaveError => {
                let ss: AuroraErrorInfo = Error::SaveError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProjectErrorDefinesNotNull => {
                let ss: AuroraErrorInfo = Error::DeleteProjectErrorDefinesNotNull.into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessInstanceByIdsError => {
                let ss: AuroraErrorInfo = Error::BatchDeleteProcessInstanceByIdsError.into();
                write!(f, "{}", ss)
            }
            Error::PreviewScheduleError => {
                let ss: AuroraErrorInfo = Error::PreviewScheduleError.into();
                write!(f, "{}", ss)
            }
            Error::ParseToCronExpressionError => {
                let ss: AuroraErrorInfo = Error::ParseToCronExpressionError.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleStartTimeEndTimeSame => {
                let ss: AuroraErrorInfo = Error::ScheduleStartTimeEndTimeSame.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFail => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFail.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFailDefines => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFailDefines.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTenantByIdFailUsers => {
                let ss: AuroraErrorInfo = Error::DeleteTenantByIdFailUsers.into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupByIdFail => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupByIdFail.into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkerGroupFail => {
                let ss: AuroraErrorInfo = Error::QueryWorkerGroupFail.into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupFail => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupFail.into();
                write!(f, "{}", ss)
            }
            Error::UserDisabled => {
                let ss: AuroraErrorInfo = Error::UserDisabled.into();
                write!(f, "{}", ss)
            }
            Error::CopyProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::CopyProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::MoveProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::MoveProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionError => {
                let ss: AuroraErrorInfo = Error::SwitchProcessDefinitionVersionError.into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError => {
                let ss: AuroraErrorInfo =
                    Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError
                        .into();
                write!(f, "{}", ss)
            }
            Error::QueryProcessDefinitionVersionsError => {
                let ss: AuroraErrorInfo = Error::QueryProcessDefinitionVersionsError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefinitionVersionError => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefinitionVersionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryUserCreatedProjectError => {
                let ss: AuroraErrorInfo = Error::QueryUserCreatedProjectError.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionCodesIsEmpty => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionCodesIsEmpty.into();
                write!(f, "{}", ss)
            }
            Error::BatchCopyProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::BatchCopyProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::BatchMoveProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::BatchMoveProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkflowLineageError => {
                let ss: AuroraErrorInfo = Error::QueryWorkflowLineageError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedAndUserCreatedProjectError => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedAndUserCreatedProjectError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefinitionByCodeFail => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefinitionByCodeFail.into();
                write!(f, "{}", ss)
            }
            Error::CheckOsTenantCodeError => {
                let ss: AuroraErrorInfo = Error::CheckOsTenantCodeError.into();
                write!(f, "{}", ss)
            }
            Error::ForceTaskSuccessError => {
                let ss: AuroraErrorInfo = Error::ForceTaskSuccessError.into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceStateOperationError => {
                let ss: AuroraErrorInfo = Error::TaskInstanceStateOperationError.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceTypeNotExist => {
                let ss: AuroraErrorInfo = Error::DatasourceTypeNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionNameExist => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionNameExist.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceDbTypeIllegal => {
                let ss: AuroraErrorInfo = Error::DatasourceDbTypeIllegal.into();
                write!(f, "{}", ss)
            }
            Error::DatasourcePortIllegal => {
                let ss: AuroraErrorInfo = Error::DatasourcePortIllegal.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceOtherParamsIllegal => {
                let ss: AuroraErrorInfo = Error::DatasourceOtherParamsIllegal.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceNameIllegal => {
                let ss: AuroraErrorInfo = Error::DatasourceNameIllegal.into();
                write!(f, "{}", ss)
            }
            Error::DatasourceHostIllegal => {
                let ss: AuroraErrorInfo = Error::DatasourceHostIllegal.into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupNotExist => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupNotExist.into();
                write!(f, "{}", ss)
            }
            Error::CreateWorkerGroupForbiddenInDocker => {
                let ss: AuroraErrorInfo = Error::CreateWorkerGroupForbiddenInDocker.into();
                write!(f, "{}", ss)
            }
            Error::DeleteWorkerGroupForbiddenInDocker => {
                let ss: AuroraErrorInfo = Error::DeleteWorkerGroupForbiddenInDocker.into();
                write!(f, "{}", ss)
            }
            Error::WorkerAddressInvalid => {
                let ss: AuroraErrorInfo = Error::WorkerAddressInvalid.into();
                write!(f, "{}", ss)
            }
            Error::QueryWorkerAddressListFail => {
                let ss: AuroraErrorInfo = Error::QueryWorkerAddressListFail.into();
                write!(f, "{}", ss)
            }
            Error::TransformProjectOwnership => {
                let ss: AuroraErrorInfo = Error::TransformProjectOwnership.into();
                write!(f, "{}", ss)
            }
            Error::QueryAlertGroupError => {
                let ss: AuroraErrorInfo = Error::QueryAlertGroupError.into();
                write!(f, "{}", ss)
            }
            Error::CurrentLoginUserTenantNotExist => {
                let ss: AuroraErrorInfo = Error::CurrentLoginUserTenantNotExist.into();
                write!(f, "{}", ss)
            }
            Error::RevokeProjectError => {
                let ss: AuroraErrorInfo = Error::RevokeProjectError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedUser => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedUser.into();
                write!(f, "{}", ss)
            }
            Error::ProjectNotExist => {
                let ss: AuroraErrorInfo = Error::ProjectNotExist.into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceHostIsNull => {
                let ss: AuroraErrorInfo = Error::TaskInstanceHostIsNull.into();
                write!(f, "{}", ss)
            }
            Error::QueryExecutingWorkflowError => {
                let ss: AuroraErrorInfo = Error::QueryExecutingWorkflowError.into();
                write!(f, "{}", ss)
            }
            Error::UdfFunctionNotExist => {
                let ss: AuroraErrorInfo = Error::UdfFunctionNotExist.into();
                write!(f, "{}", ss)
            }
            Error::UdfFunctionExists => {
                let ss: AuroraErrorInfo = Error::UdfFunctionExists.into();
                write!(f, "{}", ss)
            }
            Error::ResourceNotExist => {
                let ss: AuroraErrorInfo = Error::ResourceNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ResourceExist => {
                let ss: AuroraErrorInfo = Error::ResourceExist.into();
                write!(f, "{}", ss)
            }
            Error::ResourceSuffixNotSupportView => {
                let ss: AuroraErrorInfo = Error::ResourceSuffixNotSupportView.into();
                write!(f, "{}", ss)
            }
            Error::ResourceSizeExceedLimit => {
                let ss: AuroraErrorInfo = Error::ResourceSizeExceedLimit.into();
                write!(f, "{}", ss)
            }
            Error::ResourceSuffixForbidChange => {
                let ss: AuroraErrorInfo = Error::ResourceSuffixForbidChange.into();
                write!(f, "{}", ss)
            }
            Error::UdfResourceSuffixNotJar => {
                let ss: AuroraErrorInfo = Error::UdfResourceSuffixNotJar.into();
                write!(f, "{}", ss)
            }
            Error::HdfsCopyFail => {
                let ss: AuroraErrorInfo = Error::HdfsCopyFail.into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileExist => {
                let ss: AuroraErrorInfo = Error::ResourceFileExist.into();
                write!(f, "{}", ss)
            }
            Error::ResourceFileNotExist => {
                let ss: AuroraErrorInfo = Error::ResourceFileNotExist.into();
                write!(f, "{}", ss)
            }
            Error::UdfResourceIsBound => {
                let ss: AuroraErrorInfo = Error::UdfResourceIsBound.into();
                write!(f, "{}", ss)
            }
            Error::ResourceIsUsed => {
                let ss: AuroraErrorInfo = Error::ResourceIsUsed.into();
                write!(f, "{}", ss)
            }
            Error::ParentResourceNotExist => {
                let ss: AuroraErrorInfo = Error::ParentResourceNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ResourceNotExistOrNoPermission => {
                let ss: AuroraErrorInfo = Error::ResourceNotExistOrNoPermission.into();
                write!(f, "{}", ss)
            }
            Error::ResourceIsAuthorized => {
                let ss: AuroraErrorInfo = Error::ResourceIsAuthorized.into();
                write!(f, "{}", ss)
            }
            Error::UserNoOperationPerm => {
                let ss: AuroraErrorInfo = Error::UserNoOperationPerm.into();
                write!(f, "{}", ss)
            }
            Error::UserNoOperationProjectPerm => {
                let ss: AuroraErrorInfo = Error::UserNoOperationProjectPerm.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceNotExist => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceExist => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotExist => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotRelease => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotRelease.into();
                write!(f, "{}", ss)
            }
            Error::SubProcessDefineNotRelease => {
                let ss: AuroraErrorInfo = Error::SubProcessDefineNotRelease.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceAlreadyChanged => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceAlreadyChanged.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceStateOperationError => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceStateOperationError.into();
                write!(f, "{}", ss)
            }
            Error::SubProcessInstanceNotExist => {
                let ss: AuroraErrorInfo = Error::SubProcessInstanceNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineNotAllowedEdit => {
                let ss: AuroraErrorInfo = Error::ProcessDefineNotAllowedEdit.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceExecutingCommand => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceExecutingCommand.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceNotSubProcessInstance => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceNotSubProcessInstance.into();
                write!(f, "{}", ss)
            }
            Error::TaskInstanceStateCountError => {
                let ss: AuroraErrorInfo = Error::TaskInstanceStateCountError.into();
                write!(f, "{}", ss)
            }
            Error::CountProcessInstanceStateError => {
                let ss: AuroraErrorInfo = Error::CountProcessInstanceStateError.into();
                write!(f, "{}", ss)
            }
            Error::CountProcessDefinitionUserError => {
                let ss: AuroraErrorInfo = Error::CountProcessDefinitionUserError.into();
                write!(f, "{}", ss)
            }
            Error::StartProcessInstanceError => {
                let ss: AuroraErrorInfo = Error::StartProcessInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::BatchStartProcessInstanceError => {
                let ss: AuroraErrorInfo = Error::BatchStartProcessInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::ProcessInstanceError => {
                let ss: AuroraErrorInfo = Error::ProcessInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::ExecuteProcessInstanceError => {
                let ss: AuroraErrorInfo = Error::ExecuteProcessInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::CheckProcessDefinitionError => {
                let ss: AuroraErrorInfo = Error::CheckProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryRecipientsAndCopyersByProcessDefinitionError => {
                let ss: AuroraErrorInfo =
                    Error::QueryRecipientsAndCopyersByProcessDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::DataIsNotValid => {
                let ss: AuroraErrorInfo = Error::DataIsNotValid.into();
                write!(f, "{}", ss)
            }
            Error::DataIsNull => {
                let ss: AuroraErrorInfo = Error::DataIsNull.into();
                write!(f, "{}", ss)
            }
            Error::ProcessNodeHasCycle => {
                let ss: AuroraErrorInfo = Error::ProcessNodeHasCycle.into();
                write!(f, "{}", ss)
            }
            Error::ProcessNodeSParameterInvalid => {
                let ss: AuroraErrorInfo = Error::ProcessNodeSParameterInvalid.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefineStateOnline => {
                let ss: AuroraErrorInfo = Error::ProcessDefineStateOnline.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefineByCodeError => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefineByCodeError.into();
                write!(f, "{}", ss)
            }
            Error::ScheduleCronStateOnline => {
                let ss: AuroraErrorInfo = Error::ScheduleCronStateOnline.into();
                write!(f, "{}", ss)
            }
            Error::DeleteScheduleCronByIdError => {
                let ss: AuroraErrorInfo = Error::DeleteScheduleCronByIdError.into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessDefineError => {
                let ss: AuroraErrorInfo = Error::BatchDeleteProcessDefineError.into();
                write!(f, "{}", ss)
            }
            Error::BatchDeleteProcessDefineByCodesError => {
                let ss: AuroraErrorInfo = Error::BatchDeleteProcessDefineByCodesError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteProcessDefineByCodesError => {
                let ss: AuroraErrorInfo = Error::DeleteProcessDefineByCodesError.into();
                write!(f, "{}", ss)
            }
            Error::TenantNotSuitable => {
                let ss: AuroraErrorInfo = Error::TenantNotSuitable.into();
                write!(f, "{}", ss)
            }
            Error::ExportProcessDefineByIdError => {
                let ss: AuroraErrorInfo = Error::ExportProcessDefineByIdError.into();
                write!(f, "{}", ss)
            }
            Error::BatchExportProcessDefineByIdsError => {
                let ss: AuroraErrorInfo = Error::BatchExportProcessDefineByIdsError.into();
                write!(f, "{}", ss)
            }
            Error::ImportProcessDefineError => {
                let ss: AuroraErrorInfo = Error::ImportProcessDefineError.into();
                write!(f, "{}", ss)
            }
            Error::TaskDefineNotExist => {
                let ss: AuroraErrorInfo = Error::RequestParamsNotValidError.into();
                write!(f, "{}", ss)
            }
            Error::CreateProcessTaskRelationError => {
                let ss: AuroraErrorInfo = Error::CreateProcessTaskRelationError.into();
                write!(f, "{}", ss)
            }
            Error::ProcessTaskRelationNotExist => {
                let ss: AuroraErrorInfo = Error::ProcessTaskRelationNotExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessTaskRelationExist => {
                let ss: AuroraErrorInfo = Error::ProcessTaskRelationExist.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDagIsEmpty => {
                let ss: AuroraErrorInfo = Error::ProcessDagIsEmpty.into();
                write!(f, "{}", ss)
            }
            Error::CheckProcessTaskRelationError => {
                let ss: AuroraErrorInfo = Error::CheckProcessTaskRelationError.into();
                write!(f, "{}", ss)
            }
            Error::CreateTaskDefinitionError => {
                let ss: AuroraErrorInfo = Error::CreateTaskDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateTaskDefinitionError => {
                let ss: AuroraErrorInfo = Error::UpdateTaskDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskDefinitionVersionsError => {
                let ss: AuroraErrorInfo = Error::QueryTaskDefinitionVersionsError.into();
                write!(f, "{}", ss)
            }
            Error::SwitchTaskDefinitionVersionError => {
                let ss: AuroraErrorInfo = Error::SwitchTaskDefinitionVersionError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskDefinitionVersionError => {
                let ss: AuroraErrorInfo = Error::DeleteTaskDefinitionVersionError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskDefineByCodeError => {
                let ss: AuroraErrorInfo = Error::DeleteTaskDefineByCodeError.into();
                write!(f, "{}", ss)
            }
            Error::QueryDetailOfTaskDefinitionError => {
                let ss: AuroraErrorInfo = Error::QueryDetailOfTaskDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskDefinitionListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryTaskDefinitionListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::TaskDefinitionNameExisted => {
                let ss: AuroraErrorInfo = Error::TaskDefinitionNameExisted.into();
                write!(f, "{}", ss)
            }
            Error::ReleaseTaskDefinitionError => {
                let ss: AuroraErrorInfo = Error::ReleaseTaskDefinitionError.into();
                write!(f, "{}", ss)
            }
            Error::MoveProcessTaskRelationError => {
                let ss: AuroraErrorInfo = Error::MoveProcessTaskRelationError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteTaskProcessRelationError => {
                let ss: AuroraErrorInfo = Error::DeleteTaskProcessRelationError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskProcessRelationError => {
                let ss: AuroraErrorInfo = Error::QueryTaskProcessRelationError.into();
                write!(f, "{}", ss)
            }
            Error::TaskDefineStateOnline => {
                let ss: AuroraErrorInfo = Error::TaskDefineStateOnline.into();
                write!(f, "{}", ss)
            }
            Error::TaskHasDownstream => {
                let ss: AuroraErrorInfo = Error::TaskHasDownstream.into();
                write!(f, "{}", ss)
            }
            Error::TaskHasUpstream => {
                let ss: AuroraErrorInfo = Error::TaskHasUpstream.into();
                write!(f, "{}", ss)
            }
            Error::MainTableUsingVersion => {
                let ss: AuroraErrorInfo = Error::MainTableUsingVersion.into();
                write!(f, "{}", ss)
            }
            Error::ProjectProcessNotMatch => {
                let ss: AuroraErrorInfo = Error::ProjectProcessNotMatch.into();
                write!(f, "{}", ss)
            }
            Error::DeleteEdgeError => {
                let ss: AuroraErrorInfo = Error::DeleteEdgeError.into();
                write!(f, "{}", ss)
            }
            Error::NotSupportUpdateTaskDefinition => {
                let ss: AuroraErrorInfo = Error::NotSupportUpdateTaskDefinition.into();
                write!(f, "{}", ss)
            }
            Error::NotSupportCopyTaskType => {
                let ss: AuroraErrorInfo = Error::NotSupportCopyTaskType.into();
                write!(f, "{}", ss)
            }
            Error::HdfsNotStartup => {
                let ss: AuroraErrorInfo = Error::HdfsNotStartup.into();
                write!(f, "{}", ss)
            }
            Error::StorageNotStartup => {
                let ss: AuroraErrorInfo = Error::StorageNotStartup.into();
                write!(f, "{}", ss)
            }
            Error::S3CannotRename => {
                let ss: AuroraErrorInfo = Error::S3CannotRename.into();
                write!(f, "{}", ss)
            }
            Error::QueryDatabaseStateError => {
                let ss: AuroraErrorInfo = Error::QueryDatabaseStateError.into();
                write!(f, "{}", ss)
            }
            Error::CreateAccessTokenError => {
                let ss: AuroraErrorInfo = Error::CreateAccessTokenError.into();
                write!(f, "{}", ss)
            }
            Error::GenerateTokenError => {
                let ss: AuroraErrorInfo = Error::GenerateTokenError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAccesstokenListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryAccesstokenListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateAccessTokenError => {
                let ss: AuroraErrorInfo = Error::UpdateAccessTokenError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteAccessTokenError => {
                let ss: AuroraErrorInfo = Error::DeleteAccessTokenError.into();
                write!(f, "{}", ss)
            }
            Error::AccessTokenNotExist => {
                let ss: AuroraErrorInfo = Error::AccessTokenNotExist.into();
                write!(f, "{}", ss)
            }
            Error::QueryAccesstokenByUserError => {
                let ss: AuroraErrorInfo = Error::QueryAccesstokenByUserError.into();
                write!(f, "{}", ss)
            }
            Error::CommandStateCountError => {
                let ss: AuroraErrorInfo = Error::CommandStateCountError.into();
                write!(f, "{}", ss)
            }
            Error::NegativeSizeNumberError => {
                let ss: AuroraErrorInfo = Error::NegativeSizeNumberError.into();
                write!(f, "{}", ss)
            }
            Error::StartTimeBiggerThanEndTimeError => {
                let ss: AuroraErrorInfo = Error::StartTimeBiggerThanEndTimeError.into();
                write!(f, "{}", ss)
            }
            Error::QueueCountError => {
                let ss: AuroraErrorInfo = Error::QueueCountError.into();
                write!(f, "{}", ss)
            }
            Error::KerberosStartupState => {
                let ss: AuroraErrorInfo = Error::KerberosStartupState.into();
                write!(f, "{}", ss)
            }
            Error::QueryAuditLogListPaging => {
                let ss: AuroraErrorInfo = Error::QueryAuditLogListPaging.into();
                write!(f, "{}", ss)
            }
            Error::PluginNotAUiComponent => {
                let ss: AuroraErrorInfo = Error::PluginNotAUiComponent.into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginsResultIsNull => {
                let ss: AuroraErrorInfo = Error::QueryPluginsResultIsNull.into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginsError => {
                let ss: AuroraErrorInfo = Error::QueryPluginsError.into();
                write!(f, "{}", ss)
            }
            Error::QueryPluginDetailResultIsNull => {
                let ss: AuroraErrorInfo = Error::QueryPluginDetailResultIsNull.into();
                write!(f, "{}", ss)
            }
            Error::UpdateAlertPluginInstanceError => {
                let ss: AuroraErrorInfo = Error::UpdateAlertPluginInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertPluginInstanceError => {
                let ss: AuroraErrorInfo = Error::DeleteAlertPluginInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::GetAlertPluginInstanceError => {
                let ss: AuroraErrorInfo = Error::GetAlertPluginInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::CreateAlertPluginInstanceError => {
                let ss: AuroraErrorInfo = Error::CreateAlertPluginInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAllAlertPluginInstanceError => {
                let ss: AuroraErrorInfo = Error::QueryAllAlertPluginInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::PluginInstanceAlreadyExit => {
                let ss: AuroraErrorInfo = Error::PluginInstanceAlreadyExit.into();
                write!(f, "{}", ss)
            }
            Error::ListPagingAlertPluginInstanceError => {
                let ss: AuroraErrorInfo = Error::ListPagingAlertPluginInstanceError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated => {
                let ss: AuroraErrorInfo =
                    Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated.into();
                write!(f, "{}", ss)
            }
            Error::ProcessDefinitionVersionIsUsed => {
                let ss: AuroraErrorInfo = Error::ProcessDefinitionVersionIsUsed.into();
                write!(f, "{}", ss)
            }
            Error::CreateEnvironmentError => {
                let ss: AuroraErrorInfo = Error::CreateEnvironmentError.into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentNameExists => {
                let ss: AuroraErrorInfo = Error::EnvironmentNameExists.into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentNameIsNull => {
                let ss: AuroraErrorInfo = Error::EnvironmentNameIsNull.into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentConfigIsNull => {
                let ss: AuroraErrorInfo = Error::EnvironmentConfigIsNull.into();
                write!(f, "{}", ss)
            }
            Error::UpdateEnvironmentError => {
                let ss: AuroraErrorInfo = Error::UpdateEnvironmentError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteEnvironmentError => {
                let ss: AuroraErrorInfo = Error::DeleteEnvironmentError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteEnvironmentRelatedTaskExists => {
                let ss: AuroraErrorInfo = Error::DeleteEnvironmentRelatedTaskExists.into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentByNameError => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentByNameError.into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentByCodeError => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentByCodeError.into();
                write!(f, "{}", ss)
            }
            Error::QueryEnvironmentError => {
                let ss: AuroraErrorInfo = Error::QueryEnvironmentError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyEnvironmentError => {
                let ss: AuroraErrorInfo = Error::VerifyEnvironmentError.into();
                write!(f, "{}", ss)
            }
            Error::GetRuleFormCreateJsonError => {
                let ss: AuroraErrorInfo = Error::GetRuleFormCreateJsonError.into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryRuleListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleListError => {
                let ss: AuroraErrorInfo = Error::QueryRuleListError.into();
                write!(f, "{}", ss)
            }
            Error::QueryRuleInputEntryListError => {
                let ss: AuroraErrorInfo = Error::QueryRuleInputEntryListError.into();
                write!(f, "{}", ss)
            }
            Error::QueryExecuteResultListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryExecuteResultListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceOptionsError => {
                let ss: AuroraErrorInfo = Error::GetDatasourceOptionsError.into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceTablesError => {
                let ss: AuroraErrorInfo = Error::GetDatasourceTablesError.into();
                write!(f, "{}", ss)
            }
            Error::GetDatasourceTableColumnsError => {
                let ss: AuroraErrorInfo = Error::GetDatasourceTableColumnsError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupNameExist => {
                let ss: AuroraErrorInfo = Error::TaskGroupNameExist.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupSizeError => {
                let ss: AuroraErrorInfo = Error::TaskGroupSizeError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusError => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupFull => {
                let ss: AuroraErrorInfo = Error::TaskGroupFull.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupUsedSizeError => {
                let ss: AuroraErrorInfo = Error::TaskGroupUsedSizeError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueReleaseError => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueReleaseError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueAwakeError => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueAwakeError.into();
                write!(f, "{}", ss)
            }
            Error::CreateTaskGroupError => {
                let ss: AuroraErrorInfo = Error::CreateTaskGroupError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateTaskGroupError => {
                let ss: AuroraErrorInfo = Error::UpdateTaskGroupError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskGroupListError => {
                let ss: AuroraErrorInfo = Error::QueryTaskGroupListError.into();
                write!(f, "{}", ss)
            }
            Error::CloseTaskGroupError => {
                let ss: AuroraErrorInfo = Error::CloseTaskGroupError.into();
                write!(f, "{}", ss)
            }
            Error::StartTaskGroupError => {
                let ss: AuroraErrorInfo = Error::StartTaskGroupError.into();
                write!(f, "{}", ss)
            }
            Error::QueryTaskGroupQueueListError => {
                let ss: AuroraErrorInfo = Error::QueryTaskGroupQueueListError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupCacheStartFailed => {
                let ss: AuroraErrorInfo = Error::TaskGroupCacheStartFailed.into();
                write!(f, "{}", ss)
            }
            Error::EnvironmentWorkerGroupsIsInvalid => {
                let ss: AuroraErrorInfo = Error::EnvironmentWorkerGroupsIsInvalid.into();
                write!(f, "{}", ss)
            }
            Error::UpdateEnvironmentWorkerGroupRelationError => {
                let ss: AuroraErrorInfo = Error::UpdateEnvironmentWorkerGroupRelationError.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupQueueAlreadyStart => {
                let ss: AuroraErrorInfo = Error::TaskGroupQueueAlreadyStart.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusClosed => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusClosed.into();
                write!(f, "{}", ss)
            }
            Error::TaskGroupStatusOpened => {
                let ss: AuroraErrorInfo = Error::TaskGroupStatusOpened.into();
                write!(f, "{}", ss)
            }
            Error::NotAllowToDisableOwnAccount => {
                let ss: AuroraErrorInfo = Error::NotAllowToDisableOwnAccount.into();
                write!(f, "{}", ss)
            }
            Error::NotAllowToDeleteDefaultAlarmGroup => {
                let ss: AuroraErrorInfo = Error::NotAllowToDeleteDefaultAlarmGroup.into();
                write!(f, "{}", ss)
            }
            Error::TimeZoneIllegal => {
                let ss: AuroraErrorInfo = Error::TimeZoneIllegal.into();
                write!(f, "{}", ss)
            }
            Error::QueryK8sNamespaceListPagingError => {
                let ss: AuroraErrorInfo = Error::QueryK8sNamespaceListPagingError.into();
                write!(f, "{}", ss)
            }
            Error::K8sNamespaceExist => {
                let ss: AuroraErrorInfo = Error::K8sNamespaceExist.into();
                write!(f, "{}", ss)
            }
            Error::CreateK8sNamespaceError => {
                let ss: AuroraErrorInfo = Error::CreateK8sNamespaceError.into();
                write!(f, "{}", ss)
            }
            Error::UpdateK8sNamespaceError => {
                let ss: AuroraErrorInfo = Error::UpdateK8sNamespaceError.into();
                write!(f, "{}", ss)
            }
            Error::K8sNamespaceNotExist => {
                let ss: AuroraErrorInfo = Error::K8sNamespaceNotExist.into();
                write!(f, "{}", ss)
            }
            Error::K8sClientOpsError => {
                let ss: AuroraErrorInfo = Error::K8sClientOpsError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyK8sNamespaceError => {
                let ss: AuroraErrorInfo = Error::VerifyK8sNamespaceError.into();
                write!(f, "{}", ss)
            }
            Error::DeleteK8sNamespaceByIdError => {
                let ss: AuroraErrorInfo = Error::DeleteK8sNamespaceByIdError.into();
                write!(f, "{}", ss)
            }
            Error::VerifyParameterNameFailed => {
                let ss: AuroraErrorInfo = Error::VerifyParameterNameFailed.into();
                write!(f, "{}", ss)
            }
            Error::StoreOperateCreateError => {
                let ss: AuroraErrorInfo = Error::StoreOperateCreateError.into();
                write!(f, "{}", ss)
            }
            Error::GrantK8sNamespaceError => {
                let ss: AuroraErrorInfo = Error::GrantK8sNamespaceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryUnauthorizedNamespaceError => {
                let ss: AuroraErrorInfo = Error::QueryUnauthorizedNamespaceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryAuthorizedNamespaceError => {
                let ss: AuroraErrorInfo = Error::QueryAuthorizedNamespaceError.into();
                write!(f, "{}", ss)
            }
            Error::QueryCanUseK8sClusterError => {
                let ss: AuroraErrorInfo = Error::QueryCanUseK8sClusterError.into();
                write!(f, "{}", ss)
            }
            Error::ResourceFullNameTooLongError => {
                let ss: AuroraErrorInfo = Error::ResourceFullNameTooLongError.into();
                write!(f, "{}", ss)
            }
            Error::TenantFullNameTooLongError => {
                let ss: AuroraErrorInfo = Error::TenantFullNameTooLongError.into();
                write!(f, "{}", ss)
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuroraErrorInfo {
    pub code: i32,
    // #[cfg(feature = "en_msg")]
    pub en_msg: String,
    // #[cfg(feature = "cn_msg")]
    pub cn_msg: String,
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
            .split(':')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"10000")
            .parse::<i32>()
            .unwrap_or(10000);
        let en_msg = code_info[1]
            .split(':')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"internal server error please check the log")
            .to_string();
        let cn_msg = code_info[2]
            .split(':')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"内部服务错误，请查看日志")
            .to_string();
        Ok(AuroraErrorInfo::new(*code, en_msg, cn_msg))
    }
}

impl From<String> for AuroraErrorInfo {
    fn from(msg: String) -> Self {
        let code_info: Vec<_> = msg.split('~').collect();
        let code = &code_info[0]
            .split(':')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"10000")
            .parse::<i32>()
            .unwrap_or(10000);
        let en_msg = code_info[1]
            .split(':')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"internal server error please check the log")
            .to_string();
        let cn_msg = code_info[2]
            .split(':')
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"内部服务错误，请查看日志")
            .to_string();
        AuroraErrorInfo::new(*code, en_msg, cn_msg)
    }
}

impl std::error::Error for AuroraErrorInfo {}
impl core::fmt::Display for AuroraErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "code:{}~en_msg:{}~cn_msg:{}",
            self.code, self.en_msg, self.cn_msg
        )
    }
}

impl Default for AuroraErrorInfo {
    fn default() -> Self {
        Self {
            code: 0,
            en_msg: "success".to_string(),
            cn_msg: "成功".to_string(),
        }
    }
}

impl From<AuroraErrorInfo> for Error {
    fn from(value: AuroraErrorInfo) -> Self {
        match (value.code, value.en_msg.as_str(), value.cn_msg.as_str()) {
            (0, ..) => Error::SUCCESS,
            (10000, ..) => Error::InternalServerErrorArgs,
            (10001, ..) => Error::RequestParamsNotValidError,
            (10002, ..) => Error::TaskTimeoutParamsError,
            (10003, ..) => Error::UserNameExist,           //,
            (10004, ..) => Error::UserNameNull,            //,
            (10006, ..) => Error::HdfsOperationError, //(10006, "hdfs operation error", "hdfs操作错误"),
            (10008, ..) => Error::TaskInstanceNotFound, //(10008, "task instance not found", "任务实例不存在"),
            (10009, ..) => Error::OsTenantCodeExist, //(10009, "os tenant code {0} already exists", "操作系统租户[{0}]已存在"),
            (10010, ..) => Error::UserNotExist,      //,
            (10011, ..) => Error::AlertGroupNotExist, //(10011, "alarm group not found", "告警组不存在"),
            (10012, ..) => Error::AlertGroupExist, //(10012, "alarm group already exists", "告警组名称已存在"),
            (10013, ..) => Error::UserNamePasswdError, //(10013, "user name or password error", "用户名或密码错误"),
            (10014, ..) => Error::LoginSessionFailed, //(10014, "create session failed!", "创建session失败"),
            (10015, ..) => Error::DatasourceExist, //(10015, "data source name already exists", "数据源名称已存在"),
            (10016, ..) => Error::DatasourceConnectFailed, //(10016, "data source connection failed", "建立数据源连接失败"),
            (10017, ..) => Error::TenantNotExist, //(10017, "tenant not exists", "租户不存在"),
            (10018, ..) => Error::ProjectNotFound, //(10018, "project {0} not found ", "项目[{0}]不存在"),
            (10019, ..) => Error::ProjectAlreadyExists, //(10019, "project {0} already exists", "项目名称[{0}]已存在"),
            (10020, ..) => Error::TaskInstanceNotExists, //(10020, "task instance {0} does not exist", "任务实例[{0}]不存在"),
            (10021, ..) => Error::TaskInstanceNotSubWorkflowInstance, //(10021, "task instance {0} is not sub process instance", "任务实例[{0}]不是子流程实例"),
            (10022, ..) => Error::ScheduleCronNotExists, //(10022, "scheduler crontab {0} does not exist", "调度配置定时表达式[{0}]不存在"),
            (10023, ..) => Error::ScheduleCronOnlineForbidUpdate, //(10023, "online status does not allow update operations", "调度配置上线状态不允许修改"),
            (10024, ..) => Error::ScheduleCronCheckFailed, //(10024, "scheduler crontab expression validation failure: {0}", "调度配置定时表达式验证失败: {0}"),
            (10025, ..) => Error::MasterNotExists, //(10025, "master does not exist", "无可用master节点"),
            (10026, ..) => Error::ScheduleStatusUnknown, //(10026, "unknown status: {0}", "未知状态: {0}"),
            (10027, ..) => Error::CreateAlertGroupError, //(10027, "create alert group error", "创建告警组错误"),
            (10028, ..) => Error::QueryAllAlertgroupError, //(10028, "query all alertgroup error", "查询告警组错误"),
            (10029, ..) => Error::ListPagingAlertGroupError, //(10029, "list paging alert group error", "分页查询告警组错误"),
            (10030, ..) => Error::UpdateAlertGroupError, //(10030, "update alert group error", "更新告警组错误"),
            (10031, ..) => Error::DeleteAlertGroupError, //(10031, "delete alert group error", "删除告警组错误"),
            (10032, ..) => Error::AlertGroupGrantUserError, //(10032, "alert group grant user error", "告警组授权用户错误"),
            (10033, ..) => Error::CreateDatasourceError, //(10033, "create datasource error", "创建数据源错误"),
            (10034, ..) => Error::UpdateDatasourceError, //(10034, "update datasource error", "更新数据源错误"),
            (10035, ..) => Error::QueryDatasourceError, //(10035, "query datasource error", "查询数据源错误"),
            (10036, ..) => Error::ConnectDatasourceFailure, //(10036, "connect datasource failure", "建立数据源连接失败"),
            (10037, ..) => Error::ConnectionTestFailure, //(10037, "connection test failure", "测试数据源连接失败"),
            (10038, ..) => Error::DeleteDataSourceFailure, //(10038, "delete data source failure", "删除数据源失败"),
            (10039, ..) => Error::VerifyDatasourceNameFailure, //(10039, "verify datasource name failure", "验证数据源名称失败"),
            (10040, ..) => Error::UnauthorizedDatasource, //(10040, "unauthorized datasource", "未经授权的数据源"),
            (10041, ..) => Error::AuthorizedDataSource, //(10041, "authorized data source", "授权数据源失败"),
            (10042, ..) => Error::LoginSuccess,         //(10042, "login success", "登录成功"),
            (10043, ..) => Error::UserLoginFailure, //(10043, "user login failure", "用户登录失败"),
            (10044, ..) => Error::ListWorkersError, //(10044, "list workers error", "查询worker列表错误"),
            (10045, ..) => Error::ListMastersError, //(10045, "list masters error", "查询master列表错误"),
            (10046, ..) => Error::UpdateProjectError, //(10046, "update project error", "更新项目信息错误"),
            (10047, ..) => Error::QueryProjectDetailsByCodeError, //(10047, "query project details by code error", "查询项目详细信息错误"),
            (10048, ..) => Error::CreateProjectError, //(10048, "create project error", "创建项目错误"),
            (10049, ..) => Error::LoginUserQueryProjectListPagingError, //(10049, "login user query project list paging error", "分页查询项目列表错误"),
            (10050, ..) => Error::DeleteProjectError, //(10050, "delete project error", "删除项目错误"),
            (10051, ..) => Error::QueryUnauthorizedProjectError, //(10051, "query unauthorized project error", "查询未授权项目错误"),
            (10052, ..) => Error::QueryAuthorizedProject, //(10052, "query authorized project", "查询授权项目错误"),
            (10053, ..) => Error::QueryQueueListError, //(10053, "query queue list error", "查询队列列表错误"),
            (10054, ..) => Error::CreateResourceError, //(10054, "create resource error", "创建资源错误"),
            (10055, ..) => Error::UpdateResourceError, //(10055, "update resource error", "更新资源错误"),
            (10056, ..) => Error::QueryResourcesListError, //(10056, "query resources list error", "查询资源列表错误"),
            (10057, "query resources list paging", "分页查询资源列表错误") => {
                Error::QueryResourcesListPaging
            } //(10057, "query resources list paging", "分页查询资源列表错误"),
            (10058, ..) => Error::DeleteResourceError, //(10058, "delete resource error", "删除资源错误"),
            (10059, ..) => Error::VerifyResourceByNameAndTypeError, //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
            (10060, ..) => Error::ViewResourceFileOnLineError, //(10060, "view resource file online error", "查看资源文件错误"),
            (10061, ..) => Error::CreateResourceFileOnLineError, //(10061, "create resource file online error", "创建资源文件错误"),
            (10062, ..) => Error::ResourceFileIsEmpty, //(10062, "resource file is empty", "资源文件内容不能为空"),
            (10063, ..) => Error::EditResourceFileOnLineError, //(10063, "edit resource file online error", "更新资源文件错误"),
            (10064, ..) => Error::DownloadResourceFileError, //(10064, "download resource file error", "下载资源文件错误"),
            (10065, ..) => Error::CreateUdfFunctionError, //(10065, "create udf function error", "创建UDF函数错误"),
            (10066, ..) => Error::ViewUdfFunctionError, //(10066, "view udf function error", "查询UDF函数错误"),
            (10067, ..) => Error::UpdateUdfFunctionError, //(10067, "update udf function error", "更新UDF函数错误"),
            (10068, ..) => Error::QueryUdfFunctionListPagingError, //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
            (10069, ..) => Error::QueryDatasourceByTypeError, //(10069, "query datasource by type error", "查询数据源信息错误"),
            (10070, ..) => Error::VerifyUdfFunctionNameError, //(10070, "verify udf function name error", "UDF函数名称验证错误"),
            (10071, ..) => Error::DeleteUdfFunctionError, //(10071, "delete udf function error", "删除UDF函数错误"),
            (10072, ..) => Error::AuthorizedFileResourceError, //(10072, "authorized file resource error", "授权资源文件错误"),
            (10073, ..) => Error::AuthorizeResourceTree, //(10073, "authorize resource tree display error", "授权资源目录树错误"),
            (10074, ..) => Error::UnauthorizedUdfFunctionError, //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
            (10075, ..) => Error::AuthorizedUdfFunctionError, //(10075, "authorized udf function error", "授权UDF函数错误"),
            (10076, ..) => Error::CreateScheduleError, //(10076, "create schedule error", "创建调度配置错误"),
            (10077, ..) => Error::UpdateScheduleError, //(10077, "update schedule error", "更新调度配置错误"),
            (10078, ..) => Error::PublishScheduleOnlineError, //(10078, "publish schedule online error", "上线调度配置错误"),
            (10079, ..) => Error::OfflineScheduleError, //(10079, "offline schedule error", "下线调度配置错误"),
            (10080, ..) => Error::QueryScheduleListPagingError, //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
            (10081, ..) => Error::QueryScheduleListError, //(10081, "query schedule list error", "查询调度配置列表错误"),
            (10082, ..) => Error::QueryTaskListPagingError, //(10082, "query task list paging error", "分页查询任务列表错误"),
            (10083, ..) => Error::QueryTaskRecordListPagingError, //(10083, "query task record list paging error", "分页查询任务记录错误"),
            (10084, ..) => Error::CreateTenantError, //(10084, "create tenant error", "创建租户错误"),
            (10085, ..) => Error::QueryTenantListPagingError, //(10085, "query tenant list paging error", "分页查询租户列表错误"),
            (10086, ..) => Error::QueryTenantListError, //(10086, "query tenant list error", "查询租户列表错误"),
            (10087, ..) => Error::UpdateTenantError, //(10087, "update tenant error", "更新租户错误"),
            (10088, ..) => Error::DeleteTenantByIdError, //(10088, "delete tenant by id error", "删除租户错误"),
            (10089, ..) => Error::VerifyOsTenantCodeError, //(10089, "verify os tenant code error", "操作系统租户验证错误"),
            (10090, ..) => Error::CreateUserError, //(10090, "create user error", "创建用户错误"),
            (10091, ..) => Error::QueryUserListPagingError, //(10091, "query user list paging error", "分页查询用户列表错误"),
            (10092, ..) => Error::UpdateUserError, //(10092, "update user error", "更新用户错误"),
            (10093, ..) => Error::DeleteUserByIdError, //(10093, "delete user by id error", "删除用户错误"),
            (10094, ..) => Error::GrantProjectError, //(10094, "grant project error", "授权项目错误"),
            (10095, ..) => Error::GrantResourceError, //(10095, "grant resource error", "授权资源错误"),
            (10096, ..) => Error::GrantUdfFunctionError, //(10096, "grant udf function error", "授权UDF函数错误"),
            (10097, ..) => Error::GrantDatasourceError, //(10097, "grant datasource error", "授权数据源错误"),
            (10098, ..) => Error::GetUserInfoError, //(10098, "get user info error", "获取用户信息错误"),
            (10099, ..) => Error::UserListError, //(10099, "user list error", "查询用户列表错误"),
            (10100, ..) => Error::VerifyUsernameError, //(10100, "verify username error", "用户名验证错误"),
            (10101, ..) => Error::UnauthorizedUserError, //(10101, "unauthorized user error", "查询未授权用户错误"),
            (10102, ..) => Error::AuthorizedUserError, //(10102, "authorized user error", "查询授权用户错误"),
            (10103, ..) => Error::QueryTaskInstanceLogError, //(10103, "view task instance log error", "查询任务实例日志错误"),
            (10104, ..) => Error::DownloadTaskInstanceLogFileError, //(10104, "download task instance log file error", "下载任务日志文件错误"),
            (10105, ..) => Error::CreateProcessDefinitionError, //(10105, "create process definition error", "创建工作流错误"),
            (10106, ..) => Error::VerifyProcessDefinitionNameUniqueError, //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
            (10107, ..) => Error::UpdateProcessDefinitionError, //(10107, "update process definition error", "更新工作流定义错误"),
            (10108, ..) => Error::ReleaseProcessDefinitionError, //(10108, "release process definition error", "上线工作流错误"),
            (10109, ..) => Error::QueryDetailOfProcessDefinitionError, //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
            (10110, ..) => Error::QueryProcessDefinitionList, //(10110, "query process definition list", "查询工作流列表错误"),
            (10111, ..) => Error::EncapsulationTreeviewStructureError, //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
            (10112, ..) => Error::GetTasksListByProcessDefinitionIdError, //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
            (10113, ..) => Error::QueryProcessInstanceListPagingError, //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
            (10114, ..) => Error::QueryTaskListByProcessInstanceIdError, //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
            (10115, ..) => Error::UpdateProcessInstanceError, //(10115, "update process instance error", "更新工作流实例错误"),
            (10116, ..) => Error::QueryProcessInstanceByIdError, //(10116, "query process instance by id error", "查询工作流实例错误"),
            (10117, "delete process instance by id error", "删除工作流实例错误") => {
                Error::DeleteProcessInstanceByIdError
            } //(10117, "delete process instance by id error", "删除工作流实例错误"),
            (10118, ..) => Error::QuerySubProcessInstanceDetailInfoByTaskIdError, //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
            (10119, ..) => Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError, //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
            (10120, ..) => Error::QueryProcessInstanceAllVariablesError, //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
            (10121, ..) => Error::EncapsulationProcessInstanceGanttStructureError, //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
            (10122, ..) => Error::QueryProcessDefinitionListPagingError, //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
            (10123, ..) => Error::SignOutError, //(10123, "sign out error", "退出错误"),
            (10124, ..) => Error::OsTenantCodeHasAlreadyExists, //(10124, "os tenant code has already exists", "操作系统租户已存在"),
            (10125, ..) => Error::IpIsEmpty, //(10125, "ip is empty", "IP地址不能为空"),
            (10126, ..) => Error::ScheduleCronReleaseNeedNotChange, //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
            (10127, ..) => Error::CreateQueueError, //(10127, "create queue error", "创建队列错误"),
            (10128, ..) => Error::QueueNotExist, //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
            (10129, ..) => Error::QueueValueExist, //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
            (10130, ..) => Error::QueueNameExist, //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
            (10131, ..) => Error::UpdateQueueError, //(10131, "update queue error", "更新队列信息错误"),
            (10132, ..) => Error::NeedNotUpdateQueue, //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
            (10133, ..) => Error::VerifyQueueError, //(10133, "verify queue error", "验证队列信息错误"),
            (10134, ..) => Error::NameNull, //(10134, "name must be not null", "名称不能为空"),
            (10135, ..) => Error::NameExist, //(10135, "name {0} already exists", "名称[{0}]已存在"),
            (10136, ..) => Error::SaveError, //(10136, "save error", "保存错误"),
            (
                10117,
                "please delete the process definitions in project first!",
                "请先删除全部工作流定义",
            ) => Error::DeleteProjectErrorDefinesNotNull, //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
            (10138, ..) => Error::BatchDeleteProcessInstanceByIdsError, //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
            (10139, ..) => Error::PreviewScheduleError, //(10139, "preview schedule error", "预览调度配置错误"),
            (10140, ..) => Error::ParseToCronExpressionError, //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
            (10141, ..) => Error::ScheduleStartTimeEndTimeSame, //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
            (10142, ..) => Error::DeleteTenantByIdFail, //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
            (10143, ..) => Error::DeleteTenantByIdFailDefines, //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
            (10144, ..) => Error::DeleteTenantByIdFailUsers, //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
            (10145, ..) => Error::DeleteWorkerGroupByIdFail, //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
            (10146, ..) => Error::QueryWorkerGroupFail, //(10146, "query worker group fail ", "查询worker分组失败"),
            (10147, ..) => Error::DeleteWorkerGroupFail, //(10147, "delete worker group fail ", "删除worker分组失败"),
            (10148, ..) => Error::UserDisabled, //(10148, "The current user is disabled", "当前用户已停用"),
            (10149, ..) => Error::CopyProcessDefinitionError, //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
            (10150, ..) => Error::MoveProcessDefinitionError, //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
            (10151, ..) => Error::SwitchProcessDefinitionVersionError, //(10151, "Switch process definition version error", "切换工作流版本出错"),
            (10152, ..) => Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError, //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
            (10153, ..) => {
                Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError
            } //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
            (10154, ..) => Error::QueryProcessDefinitionVersionsError, //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
            (10156, ..) => Error::DeleteProcessDefinitionVersionError, //(10156, "delete process definition version error", "删除工作流历史版本出错"),

            (10157, ..) => Error::QueryUserCreatedProjectError, //(10157, "query user created project error error", "查询用户创建的项目错误"),
            (10158, ..) => Error::ProcessDefinitionCodesIsEmpty, //(10158, "process definition codes is empty", "工作流CODES不能为空"),
            (10159, ..) => Error::BatchCopyProcessDefinitionError, //(10159, "batch copy process definition error", "复制工作流错误"),
            (10160, ..) => Error::BatchMoveProcessDefinitionError, //(10160, "batch move process definition error", "移动工作流错误"),
            (10161, ..) => Error::QueryWorkflowLineageError, //(10161, "query workflow lineage error", "查询血缘失败"),
            (10162, ..) => Error::QueryAuthorizedAndUserCreatedProjectError, //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
            (10163, ..) => Error::DeleteProcessDefinitionByCodeFail, //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
            (10164, ..) => Error::CheckOsTenantCodeError, //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
            (10165, ..) => Error::ForceTaskSuccessError, //(10165, "force task success error", "强制成功任务实例错误"),
            (10166, ..) => Error::TaskInstanceStateOperationError, //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
            (10167, ..) => Error::DatasourceTypeNotExist, //(10167, "data source type not exist", "数据源类型不存在"),
            (10168, ..) => Error::ProcessDefinitionNameExist, //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
            (10169, ..) => Error::DatasourceDbTypeIllegal, //(10169, "datasource type illegal", "数据源类型参数不合法"),
            (10170, ..) => Error::DatasourcePortIllegal, //(10170, "datasource port illegal", "数据源端口参数不合法"),
            (10171, ..) => Error::DatasourceOtherParamsIllegal, //(10171, "datasource other params illegal", "数据源其他参数不合法"),
            (10172, ..) => Error::DatasourceNameIllegal, //(10172, "datasource name illegal", "数据源名称不合法"),
            (10173, ..) => Error::DatasourceHostIllegal, //(10173, "datasource host illegal", "数据源HOST不合法"),
            (10174, ..) => Error::DeleteWorkerGroupNotExist, //(10174, "delete worker group not exist ", "删除worker分组不存在"),
            (10175, ..) => Error::CreateWorkerGroupForbiddenInDocker, //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
            (10176, ..) => Error::DeleteWorkerGroupForbiddenInDocker, //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
            (10177, ..) => Error::WorkerAddressInvalid, //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
            (10178, ..) => Error::QueryWorkerAddressListFail, //(10178, "query worker address list fail ", "查询worker地址列表失败"),
            (10179, ..) => Error::TransformProjectOwnership, //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
            (10180, ..) => Error::QueryAlertGroupError, //(10180, "query alert group error", "查询告警组错误"),
            (10181, ..) => Error::CurrentLoginUserTenantNotExist, //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
            (10182, ..) => Error::RevokeProjectError, //(10182, "revoke project error", "撤销项目授权错误"),
            (10183, ..) => Error::QueryAuthorizedUser, //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
            (10184, ..) => Error::ProjectNotExist, //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
            (10185, ..) => Error::TaskInstanceHostIsNull, //(10191, "task instance host is null", "任务实例host为空"),
            (10186, ..) => Error::QueryExecutingWorkflowError, //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

            (20001, ..) => Error::UdfFunctionNotExist, //(20001, "UDF function not found", "UDF函数不存在"),
            (20002, ..) => Error::UdfFunctionExists, //(20002, "UDF function already exists", "UDF函数已存在"),
            (20004, ..) => Error::ResourceNotExist,  //(20004, "resource not exist", "资源不存在"),
            (20005, ..) => Error::ResourceExist, //(20005, "resource already exists", "资源已存在"),
            (20006, ..) => Error::ResourceSuffixNotSupportView, //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
            (20007, ..) => Error::ResourceSizeExceedLimit, //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
            (20008, ..) => Error::ResourceSuffixForbidChange, //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
            (20009, ..) => Error::UdfResourceSuffixNotJar, //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
            (20010, ..) => Error::HdfsCopyFail, //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
            (20011, ..) => Error::ResourceFileExist, //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
            (20012, ..) => Error::ResourceFileNotExist, //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
            (20013, ..) => Error::UdfResourceIsBound, //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
            (20014, ..) => Error::ResourceIsUsed, //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
            (20015, ..) => Error::ParentResourceNotExist, //(20015, "parent resource not exist", "父资源文件不存在"),
            (20016, ..) => Error::ResourceNotExistOrNoPermission, //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
            (20017, ..) => Error::ResourceIsAuthorized, //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

            (30001, ..) => Error::UserNoOperationPerm, //(30001, "user has no operation privilege", "当前用户没有操作权限"),
            (30002, ..) => Error::UserNoOperationProjectPerm, //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

            (50001, ..) => Error::ProcessInstanceNotExist, //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
            (50002, ..) => Error::ProcessInstanceExist, //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
            (50003, ..) => Error::ProcessDefineNotExist, //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
            (
                50004,
                "process definition {0} process version {1} not online",
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态",
            ) => Error::ProcessDefineNotRelease, //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
            (50004, "exist sub process definition not online", "存在子工作流定义不是上线状态") => {
                Error::SubProcessDefineNotRelease
            } //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
            (50005, ..) => Error::ProcessInstanceAlreadyChanged, //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
            (50006, ..) => Error::ProcessInstanceStateOperationError, //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
            (50007, ..) => Error::SubProcessInstanceNotExist, //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
            (50008, ..) => Error::ProcessDefineNotAllowedEdit, //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
            (50009, ..) => Error::ProcessInstanceExecutingCommand, //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
            (50010, ..) => Error::ProcessInstanceNotSubProcessInstance, //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
            (50011, ..) => Error::TaskInstanceStateCountError, //(50011, "task instance state count error", "查询各状态任务实例数错误"),
            (50012, ..) => Error::CountProcessInstanceStateError, //(50012, "count process instance state error", "查询各状态流程实例数错误"),
            (50013, ..) => Error::CountProcessDefinitionUserError, //(50013, "count process definition user error", "查询各用户流程定义数错误"),
            (50014, "start process instance error", "运行工作流实例错误") => {
                Error::StartProcessInstanceError
            } //(50014, "start process instance error", "运行工作流实例错误"),
            (50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}") => {
                Error::BatchStartProcessInstanceError
            } //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
            (50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误") => {
                Error::ProcessInstanceError
            } //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
            (50015, "execute process instance error", "操作工作流实例错误") => {
                Error::ExecuteProcessInstanceError
            } //(50015, "execute process instance error", "操作工作流实例错误")
            (50016, "check process definition error", "工作流定义错误") => {
                Error::CheckProcessDefinitionError
            } //(50016, "check process definition error", "工作流定义错误")
            (
                50017,
                "query recipients and copyers by process definition error",
                "查询收件人和抄送人错误",
            ) => Error::QueryRecipientsAndCopyersByProcessDefinitionError, //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误")
            (50017, "data {0} not valid", "数据[{0}]无效") => Error::DataIsNotValid, //(50017, "data {0} not valid", "数据[{0}]无效")
            (50018, "data {0} is null", "数据[{0}]不能为空") => Error::DataIsNull, //(50018, "data {0} is null", "数据[{0}]不能为空")
            (50019, "process node has cycle", "流程节点间存在循环依赖") => {
                Error::ProcessNodeHasCycle
            } //(50019, "process node has cycle", "流程节点间存在循环依赖")
            (50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效") => {
                Error::ProcessNodeSParameterInvalid
            } //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效")
            (50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线") => {
                Error::ProcessDefineStateOnline
            } //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线")
            (50022, "delete process definition by code error", "删除工作流定义错误") => {
                Error::DeleteProcessDefineByCodeError
            } //(50022, "delete process definition by code error", "删除工作流定义错误")
            (50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线") => {
                Error::ScheduleCronStateOnline
            } //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线")
            (50024, "delete schedule by id error", "删除调度配置错误") => {
                Error::DeleteScheduleCronByIdError
            } //(50024, "delete schedule by id error", "删除调度配置错误")
            (50025, "batch delete process definition error", "批量删除工作流定义错误") => {
                Error::BatchDeleteProcessDefineError
            } //(50025, "batch delete process definition error", "批量删除工作流定义错误")
            (
                50026,
                "batch delete process definition by codes {0} error",
                "批量删除工作流定义[{0}]错误",
            ) => Error::BatchDeleteProcessDefineByCodesError, //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误")
            (50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误") => {
                Error::DeleteProcessDefineByCodesError
            } //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误")
            (
                50027,
                "there is not any tenant suitable, please choose a tenant available.",
                "没有合适的租户，请选择可用的租户",
            ) => Error::TenantNotSuitable, //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户")
            (50028, "export process definition by id error", "导出工作流定义错误") => {
                Error::ExportProcessDefineByIdError
            } //(50028, "export process definition by id error", "导出工作流定义错误")
            (50028, "batch export process definition by ids error", "批量导出工作流定义错误") => {
                Error::BatchExportProcessDefineByIdsError
            } //(50028, "batch export process definition by ids error", "批量导出工作流定义错误")
            (50029, "import process definition error", "导入工作流定义错误") => {
                Error::ImportProcessDefineError
            } //(50029, "import process definition error", "导入工作流定义错误")
            (50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在") => {
                Error::TaskDefineNotExist
            } //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在")
            (50032, "create process task relation error", "创建工作流任务关系错误") => {
                Error::CreateProcessTaskRelationError
            } //(50032, "create process task relation error", "创建工作流任务关系错误")
            (50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在") => {
                Error::ProcessTaskRelationNotExist
            } //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在")
            (
                50034,
                "process task relation is already exist, processCode:[{0}]",
                "工作流任务关系已存在, processCode:[{0}]",
            ) => Error::ProcessTaskRelationExist, //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]")
            (50035, "process dag is empty", "工作流dag是空") => Error::ProcessDagIsEmpty, //(50035, "process dag is empty", "工作流dag是空")
            (50036, "check process task relation error", "工作流任务关系参数错误") => {
                Error::CheckProcessTaskRelationError
            } //(50036, "check process task relation error", "工作流任务关系参数错误")
            (50037, "create task definition error", "创建任务错误") => {
                Error::CreateTaskDefinitionError
            } //(50037, "create task definition error", "创建任务错误")
            (50038, "update task definition error", "更新任务定义错误") => {
                Error::UpdateTaskDefinitionError
            } //(50038, "update task definition error", "更新任务定义错误")
            (50039, "query task definition versions error", "查询任务历史版本信息出错") => {
                Error::QueryTaskDefinitionVersionsError
            } //(50039, "query task definition versions error", "查询任务历史版本信息出错")
            (50040, "Switch task definition version error", "切换任务版本出错") => {
                Error::SwitchTaskDefinitionVersionError
            } //(50040, "Switch task definition version error", "切换任务版本出错")
            (50041, "delete task definition version error", "删除任务历史版本出错") => {
                Error::DeleteTaskDefinitionVersionError
            } //(50041, "delete task definition version error", "删除任务历史版本出错")
            (50042, "delete task definition by code error", "删除任务定义错误") => {
                Error::DeleteTaskDefineByCodeError
            } //(50042, "delete task definition by code error", "删除任务定义错误")
            (50043, "query detail of task definition error", "查询任务详细信息错误") => {
                Error::QueryDetailOfTaskDefinitionError
            } //(50043, "query detail of task definition error", "查询任务详细信息错误")
            (50044, "query task definition list paging error", "分页查询任务定义列表错误") => {
                Error::QueryTaskDefinitionListPagingError
            } //(50044, "query task definition list paging error", "分页查询任务定义列表错误")
            (50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在") => {
                Error::TaskDefinitionNameExisted
            } //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在")
            (50046, "release task definition error", "上线任务错误") => {
                Error::ReleaseTaskDefinitionError
            } //(50046, "release task definition error", "上线任务错误")
            (50047, "move process task relation error", "移动任务到其他工作流错误") => {
                Error::MoveProcessTaskRelationError
            } //(50047, "move process task relation error", "移动任务到其他工作流错误")
            (50048, "delete process task relation error", "删除工作流任务关系错误") => {
                Error::DeleteTaskProcessRelationError
            } //(50048, "delete process task relation error", "删除工作流任务关系错误")
            (50049, "query process task relation error", "查询工作流任务关系错误") => {
                Error::QueryTaskProcessRelationError
            } //(50049, "query process task relation error", "查询工作流任务关系错误")
            (50050, "task definition [{0}] is already online", "任务定义[{0}]已上线") => {
                Error::TaskDefineStateOnline
            } //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线")
            (50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖") => {
                Error::TaskHasDownstream
            } //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖")
            (50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖") => {
                Error::TaskHasUpstream
            } //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖")
            (50053, "the version that the master table is using", "主表正在使用该版本") => {
                Error::MainTableUsingVersion
            } //(50053, "the version that the master table is using", "主表正在使用该版本")
            (50054, "the project and the process is not match", "项目和工作流不匹配") => {
                Error::ProjectProcessNotMatch
            } //(50054, "the project and the process is not match", "项目和工作流不匹配")
            (50055, "delete edge error", "删除工作流任务连接线错误") => {
                Error::DeleteEdgeError
            } //(50055, "delete edge error", "删除工作流任务连接线错误")
            (50056, "task state does not support modification", "当前任务不支持修改") => {
                Error::NotSupportUpdateTaskDefinition
            } //(50056, "task state does not support modification", "当前任务不支持修改")
            (50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]") => {
                Error::NotSupportCopyTaskType
            } //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]")
            (60001, "hdfs not startup", "hdfs未启用") => Error::HdfsNotStartup, //(60001, "hdfs not startup", "hdfs未启用")
            (60002, "storage not startup", "存储未启用") => Error::StorageNotStartup, //(60002, "storage not startup", "存储未启用")
            (60003, "directory cannot be renamed", "S3无法重命名文件夹") => {
                Error::S3CannotRename
            } //(60003, "directory cannot be renamed", "S3无法重命名文件夹")
            // for monitor
            (70001, "query database state error", "查询数据库状态错误") => {
                Error::QueryDatabaseStateError
            } //(70001, "query database state error", "查询数据库状态错误")

            (70010, ..) => Error::CreateAccessTokenError, //(70010, "create access token error", "创建访问token错误")
            (70011, ..) => Error::GenerateTokenError, //(70011, "generate token error", "生成token错误")
            (70012, ..) => Error::QueryAccesstokenListPagingError, //(70012, "query access token list paging error", "分页查询访问token列表错误")
            (70013, ..) => Error::UpdateAccessTokenError, //(70013, "update access token error", "更新访问token错误")
            (70014, ..) => Error::DeleteAccessTokenError, //(70014, "delete access token error", "删除访问token错误")
            (70015, ..) => Error::AccessTokenNotExist, //(70015, "access token not exist", "访问token不存在")
            (70016, ..) => Error::QueryAccesstokenByUserError, //(70016, "query access token by user error", "查询访问指定用户的token错误")

            (80001, ..) => Error::CommandStateCountError, //(80001, "task instance state count error", "查询各状态任务实例数错误")
            (80002, ..) => Error::NegativeSizeNumberError, //(80002, "query size number error", "查询size错误")
            (80003, ..) => Error::StartTimeBiggerThanEndTimeError, //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误")
            (90001, ..) => Error::QueueCountError, //(90001, "queue count error", "查询队列数据错误")

            (100001, ..) => Error::KerberosStartupState, //(100001, "get kerberos startup state error", "获取kerberos启动状态错误")

            // audit log
            (10057, "query audit log list paging", "分页查询日志列表错误") => {
                Error::QueryAuditLogListPaging
            } //(10057, "query audit log list paging", "分页查询日志列表错误")

            //plugin
            (110001, ..) => Error::PluginNotAUiComponent, //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件")
            (110002, ..) => Error::QueryPluginsResultIsNull, //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功")
            (110003, ..) => Error::QueryPluginsError, //(110003, "query plugins error", "查询插件错误")
            (110004, ..) => Error::QueryPluginDetailResultIsNull, //(110004, "query plugin detail result is null", "查询插件详情结果为空")

            (110005, ..) => Error::UpdateAlertPluginInstanceError, //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误")
            (110006, ..) => Error::DeleteAlertPluginInstanceError, //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误")
            (110007, ..) => Error::GetAlertPluginInstanceError, //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误")
            (110008, ..) => Error::CreateAlertPluginInstanceError, //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误")
            (110009, ..) => Error::QueryAllAlertPluginInstanceError, //(110009, "query all alert plugin instance error", "查询所有告警实例失败")
            (110010, ..) => Error::PluginInstanceAlreadyExit, //(110010, "plugin instance already exit", "该告警插件实例已存在")
            (110011, ..) => Error::ListPagingAlertPluginInstanceError, //(110011, "query plugin instance page error", "分页查询告警实例失败")
            (110012, ..) => Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated, //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组")
            (110013, ..) => Error::ProcessDefinitionVersionIsUsed, //(110013, "this process definition version is used", "此工作流定义版本被使用")

            (120001, ..) => Error::CreateEnvironmentError, //(120001, "create environment error", "创建环境失败")
            (120002, ..) => Error::EnvironmentNameExists, //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在")
            (120003, ..) => Error::EnvironmentNameIsNull, //(120003, "this environment name shouldn't be empty.", "环境名称不能为空")
            (120004, ..) => Error::EnvironmentConfigIsNull, //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空")
            (120005, ..) => Error::UpdateEnvironmentError, //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败")
            (120006, ..) => Error::DeleteEnvironmentError, //(120006, "delete environment error", "删除环境信息失败")
            (120007, ..) => Error::DeleteEnvironmentRelatedTaskExists, //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息")
            (1200008, ..) => Error::QueryEnvironmentByNameError, //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在")
            (1200009, ..) => Error::QueryEnvironmentByCodeError, //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在")
            (1200010, ..) => Error::QueryEnvironmentError, //(1200010, "login user query environment error", "分页查询环境列表错误")
            (1200011, ..) => Error::VerifyEnvironmentError, //(1200011, "verify environment error", "验证环境信息错误")
            (1200012, ..) => Error::GetRuleFormCreateJsonError, //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误")
            (1200013, ..) => Error::QueryRuleListPagingError, //(1200013, "query rule list paging error", "获取规则分页列表错误")
            (1200014, ..) => Error::QueryRuleListError, //(1200014, "query rule list error", "获取规则列表错误")
            (1200015, ..) => Error::QueryRuleInputEntryListError, //(1200015, "query rule list error", "获取规则列表错误")
            (1200016, ..) => Error::QueryExecuteResultListPagingError, //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误")
            (1200017, ..) => Error::GetDatasourceOptionsError, //(1200017, "get datasource options error", "获取数据源Options错误")
            (1200018, ..) => Error::GetDatasourceTablesError, //(1200018, "get datasource tables error", "获取数据源表列表错误")
            (1200019, ..) => Error::GetDatasourceTableColumnsError, //(1200019, "get datasource table columns error", "获取数据源表列名错误")
            (130001, ..) => Error::TaskGroupNameExist, //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用")
            (130002, ..) => Error::TaskGroupSizeError, //(130002, "task group size error", "任务组大小应该为大于1的整数")
            (130003, ..) => Error::TaskGroupStatusError, //(130003, "task group status error", "任务组已经被关闭")
            (130004, ..) => Error::TaskGroupFull, //(130004, "task group is full", "任务组已经满了")
            (130005, ..) => Error::TaskGroupUsedSizeError, //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化")
            (130006, ..) => Error::TaskGroupQueueReleaseError, //(130006, "failed to release task group queue", "任务组资源释放时出现了错误")
            (130007, ..) => Error::TaskGroupQueueAwakeError, //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误")
            (130008, ..) => Error::CreateTaskGroupError, //(130008, "create task group error", "创建任务组错误")
            (130009, ..) => Error::UpdateTaskGroupError, //(130009, "update task group list error", "更新任务组错误")
            (130010, ..) => Error::QueryTaskGroupListError, //(130010, "query task group list error", "查询任务组列表错误")
            (130011, ..) => Error::CloseTaskGroupError, //(130011, "close task group error", "关闭任务组错误")
            (130012, ..) => Error::StartTaskGroupError, //(130012, "start task group error", "启动任务组错误")
            (130013, ..) => Error::QueryTaskGroupQueueListError, //(130013, "query task group queue list error", "查询任务组队列列表错误")
            (130014, ..) => Error::TaskGroupCacheStartFailed, //(130014, "cache start failed", "任务组相关的缓存启动失败")
            (130015, ..) => Error::EnvironmentWorkerGroupsIsInvalid, //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误")
            (130016, ..) => Error::UpdateEnvironmentWorkerGroupRelationError, //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中")
            (130017, ..) => Error::TaskGroupQueueAlreadyStart, //(130017, "task group queue already start", "节点已经获取任务组资源")
            (130018, ..) => Error::TaskGroupStatusClosed, //(130018, "The task group has been closed.", "任务组已经被关闭")
            (130019, ..) => Error::TaskGroupStatusOpened, //(130019, "The task group has been opened.", "任务组已经被开启")
            (130020, ..) => Error::NotAllowToDisableOwnAccount, //(130020, "Not allow to disable your own account", "不能停用自己的账号")
            (130030, ..) => Error::NotAllowToDeleteDefaultAlarmGroup, //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组")
            (130031, ..) => Error::TimeZoneIllegal, //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法")

            (1300001, ..) => Error::QueryK8sNamespaceListPagingError, //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误")
            (1300002, ..) => Error::K8sNamespaceExist, //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在")
            (1300003, ..) => Error::CreateK8sNamespaceError, //(1300003, "create k8s namespace error", "创建k8s命名空间错误")
            (1300004, ..) => Error::UpdateK8sNamespaceError, //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误")
            (1300005, ..) => Error::K8sNamespaceNotExist, //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在")
            (1300006, ..) => Error::K8sClientOpsError, //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]")
            (1300007, ..) => Error::VerifyK8sNamespaceError, //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误")
            (1300008, ..) => Error::DeleteK8sNamespaceByIdError, //(1300008, "delete k8s namespace by id error", "删除命名空间错误")
            (1300009, ..) => Error::VerifyParameterNameFailed, //(1300009, "The file name verify failed", "文件命名校验失败")
            (1300010, ..) => Error::StoreOperateCreateError, //(1300010, "create the resource failed", "存储操作失败")
            (1300011, ..) => Error::GrantK8sNamespaceError, //(1300011, "grant namespace error", "授权资源错误")
            (1300012, ..) => Error::QueryUnauthorizedNamespaceError, //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误")
            (1300013, ..) => Error::QueryAuthorizedNamespaceError, //(1300013, "query authorized namespace error", "查询授权命名空间错误")
            (1300014, ..) => Error::QueryCanUseK8sClusterError, //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误")
            (1300015, ..) => Error::ResourceFullNameTooLongError, //(1300015, "resource's fullname is too long error", "资源文件名过长")
            (1300016, ..) => Error::TenantFullNameTooLongError, //(1300016, "tenant's fullname is too long error", "租户名过长");
            (..) => Error::InternalServerErrorArgs,
        }
    }
}

impl From<Error> for AuroraErrorInfo {
    fn from(status: Error) -> Self {
        match status {
            Error::SUCCESS => AuroraErrorInfo::new(0, "success".to_string(), "成功".to_string()),
            Error::InternalServerErrorArgs => AuroraErrorInfo::new(
                10000,
                "internal server error please check the log".to_string(),
                "内部服务错误，请查看日志".to_string(),
            ),
            Error::RequestParamsNotValidError => AuroraErrorInfo::new(
                10001,
                "request parameter {0} is not valid".to_string(),
                "请求参数[{0}]无效".to_string(),
            ),
            Error::TaskTimeoutParamsError => AuroraErrorInfo::new(
                10002,
                "task timeout parameter is not valid".to_string(),
                "任务超时参数无效".to_string(),
            ),
            Error::UserNameExist => AuroraErrorInfo::new(
                10003,
                "user name already exists".to_string(),
                "用户名已存在".to_string(),
            ),
            Error::UserNameNull => AuroraErrorInfo::new(
                10004,
                "user name is null".to_string(),
                "用户名不能为空".to_string(),
            ),
            Error::HdfsOperationError => AuroraErrorInfo::new(
                10006,
                "hdfs operation error".to_string(),
                "hdfs操作错误".to_string(),
            ),
            Error::TaskInstanceNotFound => AuroraErrorInfo::new(
                10008,
                "task instance not found".to_string(),
                "任务实例不存在".to_string(),
            ),
            Error::OsTenantCodeExist => AuroraErrorInfo::new(
                10009,
                "os tenant code {0} already exists".to_string(),
                "操作系统租户[{0}]已存在".to_string(),
            ),
            Error::UserNotExist => AuroraErrorInfo::new(
                10010,
                "user {0} not exists".to_string(),
                "用户[{0}]不存在".to_string(),
            ),
            Error::AlertGroupNotExist => AuroraErrorInfo::new(
                10011,
                "alarm group not found".to_string(),
                "告警组不存在".to_string(),
            ),
            Error::AlertGroupExist => AuroraErrorInfo::new(
                10012,
                "alarm group already exists".to_string(),
                "告警组名称已存在".to_string(),
            ),
            Error::UserNamePasswdError => AuroraErrorInfo::new(
                10013,
                "user name or password error".to_string(),
                "用户名或密码错误".to_string(),
            ),
            Error::LoginSessionFailed => AuroraErrorInfo::new(
                10014,
                "create session failed!".to_string(),
                "创建session失败".to_string(),
            ),
            Error::DatasourceExist => AuroraErrorInfo::new(
                10015,
                "data source name already exists".to_string(),
                "数据源名称已存在".to_string(),
            ),
            Error::DatasourceConnectFailed => AuroraErrorInfo::new(
                10016,
                "data source connection failed".to_string(),
                "建立数据源连接失败".to_string(),
            ),
            Error::TenantNotExist => AuroraErrorInfo::new(
                10017,
                "tenant not exists".to_string(),
                "租户不存在".to_string(),
            ),
            Error::ProjectNotFound => AuroraErrorInfo::new(
                10018,
                "project {0} not found ".to_string(),
                "项目[{0}]不存在".to_string(),
            ),
            Error::ProjectAlreadyExists => AuroraErrorInfo::new(
                10019,
                "project {0} already exists".to_string(),
                "项目名称[{0}]已存在".to_string(),
            ),
            Error::TaskInstanceNotExists => AuroraErrorInfo::new(
                10020,
                "task instance {0} does not exist".to_string(),
                "任务实例[{0}]不存在".to_string(),
            ),
            Error::TaskInstanceNotSubWorkflowInstance => AuroraErrorInfo::new(
                10021,
                "task instance {0} is not sub process instance".to_string(),
                "任务实例[{0}]不是子流程实例".to_string(),
            ),
            Error::ScheduleCronNotExists => AuroraErrorInfo::new(
                10022,
                "scheduler crontab {0} does not exist".to_string(),
                "调度配置定时表达式[{0}]不存在".to_string(),
            ),
            Error::ScheduleCronOnlineForbidUpdate => AuroraErrorInfo::new(
                10023,
                "online status does not allow update operations".to_string(),
                "调度配置上线状态不允许修改".to_string(),
            ),
            Error::ScheduleCronCheckFailed => AuroraErrorInfo::new(
                10024,
                "scheduler crontab expression validation failure: {0}".to_string(),
                "调度配置定时表达式验证失败: {0}".to_string(),
            ),
            Error::MasterNotExists => AuroraErrorInfo::new(
                10025,
                "master does not exist".to_string(),
                "无可用master节点".to_string(),
            ),
            Error::ScheduleStatusUnknown => AuroraErrorInfo::new(
                10026,
                "unknown status: {0}".to_string(),
                "未知状态: {0}".to_string(),
            ),
            Error::CreateAlertGroupError => AuroraErrorInfo::new(
                10027,
                "create alert group error".to_string(),
                "创建告警组错误".to_string(),
            ),
            Error::QueryAllAlertgroupError => AuroraErrorInfo::new(
                10028,
                "query all alertgroup error".to_string(),
                "查询告警组错误".to_string(),
            ),
            Error::ListPagingAlertGroupError => AuroraErrorInfo::new(
                10029,
                "list paging alert group error".to_string(),
                "分页查询告警组错误".to_string(),
            ),
            Error::UpdateAlertGroupError => AuroraErrorInfo::new(
                10030,
                "update alert group error".to_string(),
                "更新告警组错误".to_string(),
            ),
            Error::DeleteAlertGroupError => AuroraErrorInfo::new(
                10031,
                "delete alert group error".to_string(),
                "删除告警组错误".to_string(),
            ),
            Error::AlertGroupGrantUserError => AuroraErrorInfo::new(
                10032,
                "alert group grant user error".to_string(),
                "告警组授权用户错误".to_string(),
            ),
            Error::CreateDatasourceError => AuroraErrorInfo::new(
                10033,
                "create datasource error".to_string(),
                "创建数据源错误".to_string(),
            ),
            Error::UpdateDatasourceError => AuroraErrorInfo::new(
                10034,
                "update datasource error".to_string(),
                "更新数据源错误".to_string(),
            ),
            Error::QueryDatasourceError => AuroraErrorInfo::new(
                10035,
                "query datasource error".to_string(),
                "查询数据源错误".to_string(),
            ),
            Error::ConnectDatasourceFailure => AuroraErrorInfo::new(
                10036,
                "connect datasource failure".to_string(),
                "建立数据源连接失败".to_string(),
            ),
            Error::ConnectionTestFailure => AuroraErrorInfo::new(
                10037,
                "connection test failure".to_string(),
                "测试数据源连接失败".to_string(),
            ),
            Error::DeleteDataSourceFailure => AuroraErrorInfo::new(
                10038,
                "delete data source failure".to_string(),
                "删除数据源失败".to_string(),
            ),
            Error::VerifyDatasourceNameFailure => AuroraErrorInfo::new(
                10039,
                "verify datasource name failure".to_string(),
                "验证数据源名称失败".to_string(),
            ),
            Error::UnauthorizedDatasource => AuroraErrorInfo::new(
                10040,
                "unauthorized datasource".to_string(),
                "未经授权的数据源".to_string(),
            ),
            Error::AuthorizedDataSource => AuroraErrorInfo::new(
                10041,
                "authorized data source".to_string(),
                "授权数据源失败".to_string(),
            ),
            Error::LoginSuccess => {
                AuroraErrorInfo::new(10042, "login success".to_string(), "登录成功".to_string())
            }
            Error::UserLoginFailure => AuroraErrorInfo::new(
                10043,
                "user login failure".to_string(),
                "用户登录失败".to_string(),
            ),
            Error::ListWorkersError => AuroraErrorInfo::new(
                10044,
                "list workers error".to_string(),
                "查询worker列表错误".to_string(),
            ),
            Error::ListMastersError => AuroraErrorInfo::new(
                10045,
                "list masters error".to_string(),
                "查询master列表错误".to_string(),
            ),
            Error::UpdateProjectError => AuroraErrorInfo::new(
                10046,
                "update project error".to_string(),
                "更新项目信息错误".to_string(),
            ),
            Error::QueryProjectDetailsByCodeError => AuroraErrorInfo::new(
                10047,
                "query project details by code error".to_string(),
                "查询项目详细信息错误".to_string(),
            ),
            Error::CreateProjectError => AuroraErrorInfo::new(
                10048,
                "create project error".to_string(),
                "创建项目错误".to_string(),
            ),
            Error::LoginUserQueryProjectListPagingError => AuroraErrorInfo::new(
                10049,
                "login user query project list paging error".to_string(),
                "分页查询项目列表错误".to_string(),
            ),
            Error::DeleteProjectError => AuroraErrorInfo::new(
                10050,
                "delete project error".to_string(),
                "删除项目错误".to_string(),
            ),
            Error::QueryUnauthorizedProjectError => AuroraErrorInfo::new(
                10051,
                "query unauthorized project error".to_string(),
                "查询未授权项目错误".to_string(),
            ),
            Error::QueryAuthorizedProject => AuroraErrorInfo::new(
                10052,
                "query authorized project".to_string(),
                "查询授权项目错误".to_string(),
            ),
            Error::QueryQueueListError => AuroraErrorInfo::new(
                10053,
                "query queue list error".to_string(),
                "查询队列列表错误".to_string(),
            ),
            Error::CreateResourceError => AuroraErrorInfo::new(
                10054,
                "create resource error".to_string(),
                "创建资源错误".to_string(),
            ),
            Error::UpdateResourceError => AuroraErrorInfo::new(
                10055,
                "update resource error".to_string(),
                "更新资源错误".to_string(),
            ),
            Error::QueryResourcesListError => AuroraErrorInfo::new(
                10056,
                "query resources list error".to_string(),
                "查询资源列表错误".to_string(),
            ),
            Error::QueryResourcesListPaging => AuroraErrorInfo::new(
                10057,
                "query resources list paging".to_string(),
                "分页查询资源列表错误".to_string(),
            ),
            Error::DeleteResourceError => AuroraErrorInfo::new(
                10058,
                "delete resource error".to_string(),
                "删除资源错误".to_string(),
            ),
            Error::VerifyResourceByNameAndTypeError => AuroraErrorInfo::new(
                10059,
                "verify resource by name and type error".to_string(),
                "资源名称或类型验证错误".to_string(),
            ),
            Error::ViewResourceFileOnLineError => AuroraErrorInfo::new(
                10060,
                "view resource file online error".to_string(),
                "查看资源文件错误".to_string(),
            ),
            Error::CreateResourceFileOnLineError => AuroraErrorInfo::new(
                10061,
                "create resource file online error".to_string(),
                "创建资源文件错误".to_string(),
            ),
            Error::ResourceFileIsEmpty => AuroraErrorInfo::new(
                10062,
                "resource file is empty".to_string(),
                "资源文件内容不能为空".to_string(),
            ),
            Error::EditResourceFileOnLineError => AuroraErrorInfo::new(
                10063,
                "edit resource file online error".to_string(),
                "更新资源文件错误".to_string(),
            ),
            Error::DownloadResourceFileError => AuroraErrorInfo::new(
                10064,
                "download resource file error".to_string(),
                "下载资源文件错误".to_string(),
            ),
            Error::CreateUdfFunctionError => AuroraErrorInfo::new(
                10065,
                "create udf function error".to_string(),
                "创建UDF函数错误".to_string(),
            ),
            Error::ViewUdfFunctionError => AuroraErrorInfo::new(
                10066,
                "view udf function error".to_string(),
                "查询UDF函数错误".to_string(),
            ),
            Error::UpdateUdfFunctionError => AuroraErrorInfo::new(
                10067,
                "update udf function error".to_string(),
                "更新UDF函数错误".to_string(),
            ),
            Error::QueryUdfFunctionListPagingError => AuroraErrorInfo::new(
                10068,
                "query udf function list paging error".to_string(),
                "分页查询UDF函数列表错误".to_string(),
            ),
            Error::QueryDatasourceByTypeError => AuroraErrorInfo::new(
                10069,
                "query datasource by type error".to_string(),
                "查询数据源信息错误".to_string(),
            ),
            Error::VerifyUdfFunctionNameError => AuroraErrorInfo::new(
                10070,
                "verify udf function name error".to_string(),
                "UDF函数名称验证错误".to_string(),
            ),
            Error::DeleteUdfFunctionError => AuroraErrorInfo::new(
                10071,
                "delete udf function error".to_string(),
                "删除UDF函数错误".to_string(),
            ),
            Error::AuthorizedFileResourceError => AuroraErrorInfo::new(
                10072,
                "authorized file resource error".to_string(),
                "授权资源文件错误".to_string(),
            ),
            Error::AuthorizeResourceTree => AuroraErrorInfo::new(
                10073,
                "authorize resource tree display error".to_string(),
                "授权资源目录树错误".to_string(),
            ),
            Error::UnauthorizedUdfFunctionError => AuroraErrorInfo::new(
                10074,
                "unauthorized udf function error".to_string(),
                "查询未授权UDF函数错误".to_string(),
            ),
            Error::AuthorizedUdfFunctionError => AuroraErrorInfo::new(
                10075,
                "authorized udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            ),
            Error::CreateScheduleError => AuroraErrorInfo::new(
                10076,
                "create schedule error".to_string(),
                "创建调度配置错误".to_string(),
            ),
            Error::UpdateScheduleError => AuroraErrorInfo::new(
                10077,
                "update schedule error".to_string(),
                "更新调度配置错误".to_string(),
            ),
            Error::PublishScheduleOnlineError => AuroraErrorInfo::new(
                10078,
                "publish schedule online error".to_string(),
                "上线调度配置错误".to_string(),
            ),
            Error::OfflineScheduleError => AuroraErrorInfo::new(
                10079,
                "offline schedule error".to_string(),
                "下线调度配置错误".to_string(),
            ),
            Error::QueryScheduleListPagingError => AuroraErrorInfo::new(
                10080,
                "query schedule list paging error".to_string(),
                "分页查询调度配置列表错误".to_string(),
            ),
            Error::QueryScheduleListError => AuroraErrorInfo::new(
                10081,
                "query schedule list error".to_string(),
                "查询调度配置列表错误".to_string(),
            ),
            Error::QueryTaskListPagingError => AuroraErrorInfo::new(
                10082,
                "query task list paging error".to_string(),
                "分页查询任务列表错误".to_string(),
            ),
            Error::QueryTaskRecordListPagingError => AuroraErrorInfo::new(
                10083,
                "query task record list paging error".to_string(),
                "分页查询任务记录错误".to_string(),
            ),
            Error::CreateTenantError => AuroraErrorInfo::new(
                10084,
                "create tenant error".to_string(),
                "创建租户错误".to_string(),
            ),
            Error::QueryTenantListPagingError => AuroraErrorInfo::new(
                10085,
                "query tenant list paging error".to_string(),
                "分页查询租户列表错误".to_string(),
            ),
            Error::QueryTenantListError => AuroraErrorInfo::new(
                10086,
                "query tenant list error".to_string(),
                "查询租户列表错误".to_string(),
            ),
            Error::UpdateTenantError => AuroraErrorInfo::new(
                10087,
                "update tenant error".to_string(),
                "更新租户错误".to_string(),
            ),
            Error::DeleteTenantByIdError => AuroraErrorInfo::new(
                10088,
                "delete tenant by id error".to_string(),
                "删除租户错误".to_string(),
            ),
            Error::VerifyOsTenantCodeError => AuroraErrorInfo::new(
                10089,
                "verify os tenant code error".to_string(),
                "操作系统租户验证错误".to_string(),
            ),
            Error::CreateUserError => AuroraErrorInfo::new(
                10090,
                "create user error".to_string(),
                "创建用户错误".to_string(),
            ),
            Error::QueryUserListPagingError => AuroraErrorInfo::new(
                10091,
                "query user list paging error".to_string(),
                "分页查询用户列表错误".to_string(),
            ),
            Error::UpdateUserError => AuroraErrorInfo::new(
                10092,
                "update user error".to_string(),
                "更新用户错误".to_string(),
            ),
            Error::DeleteUserByIdError => AuroraErrorInfo::new(
                10093,
                "delete user by id error".to_string(),
                "删除用户错误".to_string(),
            ),
            Error::GrantProjectError => AuroraErrorInfo::new(
                10094,
                "grant project error".to_string(),
                "授权项目错误".to_string(),
            ),
            Error::GrantResourceError => AuroraErrorInfo::new(
                10095,
                "grant resource error".to_string(),
                "授权资源错误".to_string(),
            ),
            Error::GrantUdfFunctionError => AuroraErrorInfo::new(
                10096,
                "grant udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            ),
            Error::GrantDatasourceError => AuroraErrorInfo::new(
                10097,
                "grant datasource error".to_string(),
                "授权数据源错误".to_string(),
            ),
            Error::GetUserInfoError => AuroraErrorInfo::new(
                10098,
                "get user info error".to_string(),
                "获取用户信息错误".to_string(),
            ),
            Error::UserListError => AuroraErrorInfo::new(
                10099,
                "user list error".to_string(),
                "查询用户列表错误".to_string(),
            ),
            Error::VerifyUsernameError => AuroraErrorInfo::new(
                10100,
                "verify username error".to_string(),
                "用户名验证错误".to_string(),
            ),
            Error::UnauthorizedUserError => AuroraErrorInfo::new(
                10101,
                "unauthorized user error".to_string(),
                "查询未授权用户错误".to_string(),
            ),
            Error::AuthorizedUserError => AuroraErrorInfo::new(
                10102,
                "authorized user error".to_string(),
                "查询授权用户错误".to_string(),
            ),
            Error::QueryTaskInstanceLogError => AuroraErrorInfo::new(
                10103,
                "view task instance log error".to_string(),
                "查询任务实例日志错误".to_string(),
            ),
            Error::DownloadTaskInstanceLogFileError => AuroraErrorInfo::new(
                10104,
                "download task instance log file error".to_string(),
                "下载任务日志文件错误".to_string(),
            ),
            Error::CreateProcessDefinitionError => AuroraErrorInfo::new(
                10105,
                "create process definition error".to_string(),
                "创建工作流错误".to_string(),
            ),
            Error::VerifyProcessDefinitionNameUniqueError => AuroraErrorInfo::new(
                10106,
                "verify process definition name unique error".to_string(),
                "工作流定义名称验证错误".to_string(),
            ),
            Error::UpdateProcessDefinitionError => AuroraErrorInfo::new(
                10107,
                "update process definition error".to_string(),
                "更新工作流定义错误".to_string(),
            ),
            Error::ReleaseProcessDefinitionError => AuroraErrorInfo::new(
                10108,
                "release process definition error".to_string(),
                "上线工作流错误".to_string(),
            ),
            Error::QueryDetailOfProcessDefinitionError => AuroraErrorInfo::new(
                10109,
                "query detail of process definition error".to_string(),
                "查询工作流详细信息错误".to_string(),
            ),
            Error::QueryProcessDefinitionList => AuroraErrorInfo::new(
                10110,
                "query process definition list".to_string(),
                "查询工作流列表错误".to_string(),
            ),
            Error::EncapsulationTreeviewStructureError => AuroraErrorInfo::new(
                10111,
                "encapsulation treeview structure error".to_string(),
                "查询工作流树形图数据错误".to_string(),
            ),
            Error::GetTasksListByProcessDefinitionIdError => AuroraErrorInfo::new(
                10112,
                "get tasks list by process definition id error".to_string(),
                "查询工作流定义节点信息错误".to_string(),
            ),
            Error::QueryProcessInstanceListPagingError => AuroraErrorInfo::new(
                10113,
                "query process instance list paging error".to_string(),
                "分页查询工作流实例列表错误".to_string(),
            ),
            Error::QueryTaskListByProcessInstanceIdError => AuroraErrorInfo::new(
                10114,
                "query task list by process instance id error".to_string(),
                "查询任务实例列表错误".to_string(),
            ),
            Error::UpdateProcessInstanceError => AuroraErrorInfo::new(
                10115,
                "update process instance error".to_string(),
                "更新工作流实例错误".to_string(),
            ),
            Error::QueryProcessInstanceByIdError => AuroraErrorInfo::new(
                10116,
                "query process instance by id error".to_string(),
                "查询工作流实例错误".to_string(),
            ),
            Error::DeleteProcessInstanceByIdError => AuroraErrorInfo::new(
                10117,
                "delete process instance by id error".to_string(),
                "删除工作流实例错误".to_string(),
            ),
            Error::QuerySubProcessInstanceDetailInfoByTaskIdError => AuroraErrorInfo::new(
                10118,
                "query sub process instance detail info by task id error".to_string(),
                "查询子流程任务实例错误".to_string(),
            ),
            Error::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError => {
                AuroraErrorInfo::new(
                    10119,
                    "query parent process instance detail info by sub process instance id error"
                        .to_string(),
                    "查询子流程该工作流实例错误".to_string(),
                )
            }
            Error::QueryProcessInstanceAllVariablesError => AuroraErrorInfo::new(
                10120,
                "query process instance all variables error".to_string(),
                "查询工作流自定义变量信息错误".to_string(),
            ),
            Error::EncapsulationProcessInstanceGanttStructureError => AuroraErrorInfo::new(
                10121,
                "encapsulation process instance gantt structure error".to_string(),
                "查询工作流实例甘特图数据错误".to_string(),
            ),
            Error::QueryProcessDefinitionListPagingError => AuroraErrorInfo::new(
                10122,
                "query process definition list paging error".to_string(),
                "分页查询工作流定义列表错误".to_string(),
            ),
            Error::SignOutError => {
                AuroraErrorInfo::new(10123, "sign out error".to_string(), "退出错误".to_string())
            }
            Error::OsTenantCodeHasAlreadyExists => AuroraErrorInfo::new(
                10124,
                "os tenant code has already exists".to_string(),
                "操作系统租户已存在".to_string(),
            ),
            Error::IpIsEmpty => AuroraErrorInfo::new(
                10125,
                "ip is empty".to_string(),
                "IP地址不能为空".to_string(),
            ),
            Error::ScheduleCronReleaseNeedNotChange => AuroraErrorInfo::new(
                10126,
                "schedule release is already {0}".to_string(),
                "调度配置上线错误[{0}]".to_string(),
            ),
            Error::CreateQueueError => AuroraErrorInfo::new(
                10127,
                "create queue error".to_string(),
                "创建队列错误".to_string(),
            ),
            Error::QueueNotExist => AuroraErrorInfo::new(
                10128,
                "queue {0} not exists".to_string(),
                "队列ID[{0}]不存在".to_string(),
            ),
            Error::QueueValueExist => AuroraErrorInfo::new(
                10129,
                "queue value {0} already exists".to_string(),
                "队列值[{0}]已存在".to_string(),
            ),
            Error::QueueNameExist => AuroraErrorInfo::new(
                10130,
                "queue name {0} already exists".to_string(),
                "队列名称[{0}]已存在".to_string(),
            ),
            Error::UpdateQueueError => AuroraErrorInfo::new(
                10131,
                "update queue error".to_string(),
                "更新队列信息错误".to_string(),
            ),
            Error::NeedNotUpdateQueue => AuroraErrorInfo::new(
                10132,
                "need not update queue".to_string(),
                "无需更新队列信息".to_string(),
            ),
            Error::VerifyQueueError => AuroraErrorInfo::new(
                10133,
                "verify queue error".to_string(),
                "验证队列信息错误".to_string(),
            ),
            Error::NameNull => AuroraErrorInfo::new(
                10134,
                "name must be not null".to_string(),
                "名称不能为空".to_string(),
            ),
            Error::NameExist => AuroraErrorInfo::new(
                10135,
                "name {0} already exists".to_string(),
                "名称[{0}]已存在".to_string(),
            ),
            Error::SaveError => {
                AuroraErrorInfo::new(10136, "save error".to_string(), "保存错误".to_string())
            }
            Error::DeleteProjectErrorDefinesNotNull => AuroraErrorInfo::new(
                10117,
                "please delete the process definitions in project first!".to_string(),
                "请先删除全部工作流定义".to_string(),
            ),
            Error::BatchDeleteProcessInstanceByIdsError => AuroraErrorInfo::new(
                10138,
                "batch delete process instance by ids {0} error".to_string(),
                "批量删除工作流实例错误: {0}".to_string(),
            ),
            Error::PreviewScheduleError => AuroraErrorInfo::new(
                10139,
                "preview schedule error".to_string(),
                "预览调度配置错误".to_string(),
            ),
            Error::ParseToCronExpressionError => AuroraErrorInfo::new(
                10140,
                "parse cron to cron expression error".to_string(),
                "解析调度表达式错误".to_string(),
            ),
            Error::ScheduleStartTimeEndTimeSame => AuroraErrorInfo::new(
                10141,
                "The start time must not be the same as the end".to_string(),
                "开始时间不能和结束时间一样".to_string(),
            ),
            Error::DeleteTenantByIdFail => AuroraErrorInfo::new(
                10142,
                "delete tenant by id fail:for there are {0} process instances in executing using \
                 it"
                .to_string(),
                "删除租户失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ),
            Error::DeleteTenantByIdFailDefines => AuroraErrorInfo::new(
                10143,
                "delete tenant by id fail:for there are {0} process definitions using it"
                    .to_string(),
                "删除租户失败，有[{0}]个工作流定义正在使用".to_string(),
            ),
            Error::DeleteTenantByIdFailUsers => AuroraErrorInfo::new(
                10144,
                "delete tenant by id fail: for there are {0} users using it".to_string(),
                "删除租户失败，有[{0}]个用户正在使用".to_string(),
            ),
            Error::DeleteWorkerGroupByIdFail => AuroraErrorInfo::new(
                10145,
                "delete worker group by id failfor there are {0} process instances in executing \
                 using it"
                    .to_string(),
                "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ),
            Error::QueryWorkerGroupFail => AuroraErrorInfo::new(
                10146,
                "query worker group fail ".to_string(),
                "查询worker分组失败".to_string(),
            ),
            Error::DeleteWorkerGroupFail => AuroraErrorInfo::new(
                10147,
                "delete worker group fail ".to_string(),
                "删除worker分组失败".to_string(),
            ),
            Error::UserDisabled => AuroraErrorInfo::new(
                10148,
                "The current user is disabled".to_string(),
                "当前用户已停用".to_string(),
            ),
            Error::CopyProcessDefinitionError => AuroraErrorInfo::new(
                10149,
                "copy process definition from {0} to {1} error : {2}".to_string(),
                "从{0}复制工作流到{1}错误 : {2}".to_string(),
            ),
            Error::MoveProcessDefinitionError => AuroraErrorInfo::new(
                10150,
                "move process definition from {0} to {1} error : {2}".to_string(),
                "从{0}移动工作流到{1}错误 : {2}".to_string(),
            ),
            Error::SwitchProcessDefinitionVersionError => AuroraErrorInfo::new(
                10151,
                "Switch process definition version error".to_string(),
                "切换工作流版本出错".to_string(),
            ),
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionError => {
                AuroraErrorInfo::new(
                    10152,
                    "Switch process definition version error: not exists process definition: \
                     [process definition id {0}]"
                        .to_string(),
                    "切换工作流版本出错：工作流不存在，[工作流id {0}]".to_string(),
                )
            }
            Error::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError => {
                AuroraErrorInfo::new(
                    10153,
                    "Switch process defi:nition version error: not exists process definition \
                     version: [process definition id {0}] [version number {1}]"
                        .to_string(),
                    "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"
                        .to_string(),
                )
            }
            Error::QueryProcessDefinitionVersionsError => AuroraErrorInfo::new(
                10154,
                "query process definition versions error".to_string(),
                "查询工作流历史版本信息出错".to_string(),
            ),
            Error::DeleteProcessDefinitionVersionError => AuroraErrorInfo::new(
                10156,
                "delete process definition version error".to_string(),
                "删除工作流历史版本出错".to_string(),
            ),
            Error::QueryUserCreatedProjectError => AuroraErrorInfo::new(
                10157,
                "query user created project error error".to_string(),
                "查询用户创建的项目错误".to_string(),
            ),
            Error::ProcessDefinitionCodesIsEmpty => AuroraErrorInfo::new(
                10158,
                "process definition codes is empty".to_string(),
                "工作流CODES不能为空".to_string(),
            ),
            Error::BatchCopyProcessDefinitionError => AuroraErrorInfo::new(
                10159,
                "batch copy process definition error".to_string(),
                "复制工作流错误".to_string(),
            ),
            Error::BatchMoveProcessDefinitionError => AuroraErrorInfo::new(
                10160,
                "batch move process definition error".to_string(),
                "移动工作流错误".to_string(),
            ),
            Error::QueryWorkflowLineageError => AuroraErrorInfo::new(
                10161,
                "query workflow lineage error".to_string(),
                "查询血缘失败".to_string(),
            ),
            Error::QueryAuthorizedAndUserCreatedProjectError => AuroraErrorInfo::new(
                10162,
                "query authorized and user created project error error".to_string(),
                "查询授权的和用户创建的项目错误".to_string(),
            ),
            Error::DeleteProcessDefinitionByCodeFail => AuroraErrorInfo::new(
                10163,
                "delete process definition by code fail.to_string(), for there are {0} process \
                 instances in executing using it"
                    .to_string(),
                "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ),
            Error::CheckOsTenantCodeError => AuroraErrorInfo::new(
                10164,
                "Tenant code invalid.to_string(), should follow linux's users naming conventions"
                    .to_string(),
                "非法的租户名，需要遵守 Linux 用户命名规范".to_string(),
            ),
            Error::ForceTaskSuccessError => AuroraErrorInfo::new(
                10165,
                "force task success error".to_string(),
                "强制成功任务实例错误".to_string(),
            ),
            Error::TaskInstanceStateOperationError => AuroraErrorInfo::new(
                10166,
                "the status of task instance {0} is {1}.to_string(),Cannot perform force success \
                 operation"
                    .to_string(),
                "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作".to_string(),
            ),
            Error::DatasourceTypeNotExist => AuroraErrorInfo::new(
                10167,
                "data source type not exist".to_string(),
                "数据源类型不存在".to_string(),
            ),
            Error::ProcessDefinitionNameExist => AuroraErrorInfo::new(
                10168,
                "process definition name {0} already exists".to_string(),
                "工作流定义名称[{0}]已存在".to_string(),
            ),
            Error::DatasourceDbTypeIllegal => AuroraErrorInfo::new(
                10169,
                "datasource type illegal".to_string(),
                "数据源类型参数不合法".to_string(),
            ),
            Error::DatasourcePortIllegal => AuroraErrorInfo::new(
                10170,
                "datasource port illegal".to_string(),
                "数据源端口参数不合法".to_string(),
            ),
            Error::DatasourceOtherParamsIllegal => AuroraErrorInfo::new(
                10171,
                "datasource other params illegal".to_string(),
                "数据源其他参数不合法".to_string(),
            ),
            Error::DatasourceNameIllegal => AuroraErrorInfo::new(
                10172,
                "datasource name illegal".to_string(),
                "数据源名称不合法".to_string(),
            ),
            Error::DatasourceHostIllegal => AuroraErrorInfo::new(
                10173,
                "datasource host illegal".to_string(),
                "数据源HOST不合法".to_string(),
            ),
            Error::DeleteWorkerGroupNotExist => AuroraErrorInfo::new(
                10174,
                "delete worker group not exist ".to_string(),
                "删除worker分组不存在".to_string(),
            ),
            Error::CreateWorkerGroupForbiddenInDocker => AuroraErrorInfo::new(
                10175,
                "create worker group forbidden in docker ".to_string(),
                "创建worker分组在docker中禁止".to_string(),
            ),
            Error::DeleteWorkerGroupForbiddenInDocker => AuroraErrorInfo::new(
                10176,
                "delete worker group forbidden in docker ".to_string(),
                "删除worker分组在docker中禁止".to_string(),
            ),
            Error::WorkerAddressInvalid => AuroraErrorInfo::new(
                10177,
                "worker address {0} invalid".to_string(),
                "worker地址[{0}]无效".to_string(),
            ),
            Error::QueryWorkerAddressListFail => AuroraErrorInfo::new(
                10178,
                "query worker address list fail ".to_string(),
                "查询worker地址列表失败".to_string(),
            ),
            Error::TransformProjectOwnership => AuroraErrorInfo::new(
                10179,
                "Please transform project ownership [{0}]".to_string(),
                "请先转移项目所有权[{0}]".to_string(),
            ),
            Error::QueryAlertGroupError => AuroraErrorInfo::new(
                10180,
                "query alert group error".to_string(),
                "查询告警组错误".to_string(),
            ),
            Error::CurrentLoginUserTenantNotExist => AuroraErrorInfo::new(
                10181,
                "the tenant of the currently login user is not specified".to_string(),
                "未指定当前登录用户的租户".to_string(),
            ),
            Error::RevokeProjectError => AuroraErrorInfo::new(
                10182,
                "revoke project error".to_string(),
                "撤销项目授权错误".to_string(),
            ),
            Error::QueryAuthorizedUser => AuroraErrorInfo::new(
                10183,
                "query authorized user error".to_string(),
                "查询拥有项目权限的用户错误".to_string(),
            ),
            Error::ProjectNotExist => AuroraErrorInfo::new(
                10190,
                "This project was not found. Please refresh page.".to_string(),
                "该项目不存在".to_string(),
            ),

            Error::TaskInstanceHostIsNull => AuroraErrorInfo::new(
                10191,
                "task instance host is null ".to_string(),
                "任务实例host为空".to_string(),
            ),
            Error::QueryExecutingWorkflowError => AuroraErrorInfo::new(
                10192,
                "query executing workflow error".to_string(),
                "查询运行的工作流实例错误".to_string(),
            ),
            Error::UdfFunctionNotExist => AuroraErrorInfo::new(
                20001,
                "UDF function not found".to_string(),
                "UDF函数不存在".to_string(),
            ),
            Error::UdfFunctionExists => AuroraErrorInfo::new(
                20002,
                "UDF function already exists".to_string(),
                "UDF函数已存在".to_string(),
            ),
            Error::ResourceNotExist => AuroraErrorInfo::new(
                20004,
                "resource not exist".to_string(),
                "资源不存在".to_string(),
            ),
            Error::ResourceExist => AuroraErrorInfo::new(
                20005,
                "resource already exists".to_string(),
                "资源已存在".to_string(),
            ),
            Error::ResourceSuffixNotSupportView => AuroraErrorInfo::new(
                20006,
                "resource suffix do not support online viewing".to_string(),
                "资源文件后缀不支持查看".to_string(),
            ),
            Error::ResourceSizeExceedLimit => AuroraErrorInfo::new(
                20007,
                "upload resource file size exceeds limit".to_string(),
                "上传资源文件大小超过限制".to_string(),
            ),
            Error::ResourceSuffixForbidChange => AuroraErrorInfo::new(
                20008,
                "resource suffix not allowed to be modified".to_string(),
                "资源文件后缀不支持修改".to_string(),
            ),
            Error::UdfResourceSuffixNotJar => AuroraErrorInfo::new(
                20009,
                "UDF resource suffix name must be jar".to_string(),
                "UDF资源文件后缀名只支持[jar]".to_string(),
            ),
            Error::HdfsCopyFail => AuroraErrorInfo::new(
                20010,
                "hdfs copy {0} -> {1} fail".to_string(),
                "hdfs复制失败：[{0}] -> [{1}]".to_string(),
            ),
            Error::ResourceFileExist => AuroraErrorInfo::new(
                20011,
                "resource file {0} already exists !".to_string(),
                "资源文件[{0}]已存在".to_string(),
            ),
            Error::ResourceFileNotExist => AuroraErrorInfo::new(
                20012,
                "resource file {0} not exists !".to_string(),
                "资源文件[{0}]不存在".to_string(),
            ),
            Error::UdfResourceIsBound => AuroraErrorInfo::new(
                20013,
                "udf resource file is bound by UDF functions:{0}".to_string(),
                "udf函数绑定了资源文件[{0}]".to_string(),
            ),
            Error::ResourceIsUsed => AuroraErrorInfo::new(
                20014,
                "resource file is used by process definition".to_string(),
                "资源文件被上线的流程定义使用了".to_string(),
            ),
            Error::ParentResourceNotExist => AuroraErrorInfo::new(
                20015,
                "parent resource not exist".to_string(),
                "父资源文件不存在".to_string(),
            ),
            Error::ResourceNotExistOrNoPermission => AuroraErrorInfo::new(
                20016,
                "resource not exist or no permission:please view the task node and remove error \
                 resource"
                    .to_string(),
                "请检查任务节点并移除无权限或者已删除的资源".to_string(),
            ),
            Error::ResourceIsAuthorized => AuroraErrorInfo::new(
                20017,
                "resource is authorized to user {0}:suffix not allowed to be modified".to_string(),
                "资源文件已授权其他用户[{0}]".to_string(),
            ),
            Error::UserNoOperationPerm => AuroraErrorInfo::new(
                30001,
                "user has no operation privilege".to_string(),
                "当前用户没有操作权限".to_string(),
            ),
            Error::UserNoOperationProjectPerm => AuroraErrorInfo::new(
                30002,
                "user {0} is not has project {1} permission".to_string(),
                "当前用户[{0}]没有[{1}]项目的操作权限".to_string(),
            ),
            Error::ProcessInstanceNotExist => AuroraErrorInfo::new(
                50001,
                "process instance {0} does not exist".to_string(),
                "工作流实例[{0}]不存在".to_string(),
            ),
            Error::ProcessInstanceExist => AuroraErrorInfo::new(
                50002,
                "process instance {0} already exists".to_string(),
                "工作流实例[{0}]已存在".to_string(),
            ),
            Error::ProcessDefineNotExist => AuroraErrorInfo::new(
                50003,
                "process definition {0} does not exist".to_string(),
                "工作流定义[{0}]不存在".to_string(),
            ),
            Error::ProcessDefineNotRelease => AuroraErrorInfo::new(
                50004,
                "process definition {0} process version {1} not online".to_string(),
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态".to_string(),
            ),
            Error::SubProcessDefineNotRelease => AuroraErrorInfo::new(
                50004,
                "exist sub process definition not online".to_string(),
                "存在子工作流定义不是上线状态".to_string(),
            ),
            Error::ProcessInstanceAlreadyChanged => AuroraErrorInfo::new(
                50005,
                "the status of process instance {0} is already {1}".to_string(),
                "工作流实例[{0}]的状态已经是[{1}]".to_string(),
            ),
            Error::ProcessInstanceStateOperationError => AuroraErrorInfo::new(
                50006,
                "the status of process instance {0} is {1}.to_string(),Cannot perform the \
                 operation"
                    .to_string(),
                "工作流实例[{0}]的状态是[{1}]，无法执行该操作".to_string(),
            ),
            Error::SubProcessInstanceNotExist => AuroraErrorInfo::new(
                50007,
                "the task belong to process instance does not exist".to_string(),
                "子工作流实例不存在".to_string(),
            ),
            Error::ProcessDefineNotAllowedEdit => AuroraErrorInfo::new(
                50008,
                "process definition {0} does not allow edit".to_string(),
                "工作流定义[{0}]不允许修改".to_string(),
            ),
            Error::ProcessInstanceExecutingCommand => AuroraErrorInfo::new(
                50009,
                "process instance {0} is executing command".to_string(),
                "工作流实例[{0}]正在执行命令".to_string(),
            ),
            Error::ProcessInstanceNotSubProcessInstance => AuroraErrorInfo::new(
                50010,
                "process instance {0} is not sub process instance".to_string(),
                "工作流实例[{0}]不是子工作流实例".to_string(),
            ),
            Error::TaskInstanceStateCountError => AuroraErrorInfo::new(
                50011,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            ),
            Error::CountProcessInstanceStateError => AuroraErrorInfo::new(
                50012,
                "count process instance state error".to_string(),
                "查询各状态流程实例数错误".to_string(),
            ),
            Error::CountProcessDefinitionUserError => AuroraErrorInfo::new(
                50013,
                "count process definition user error".to_string(),
                "查询各用户流程定义数错误".to_string(),
            ),
            Error::StartProcessInstanceError => AuroraErrorInfo::new(
                50014,
                "start process instance error".to_string(),
                "运行工作流实例错误".to_string(),
            ),
            Error::BatchStartProcessInstanceError => AuroraErrorInfo::new(
                50014,
                "batch start process instance error: {0}".to_string(),
                "批量运行工作流实例错误: {0}".to_string(),
            ),
            Error::ProcessInstanceError => AuroraErrorInfo::new(
                50014,
                "process instance delete error: {0}".to_string(),
                "工作流实例删除[{0}]错误".to_string(),
            ),
            Error::ExecuteProcessInstanceError => AuroraErrorInfo::new(
                50015,
                "execute process instance error".to_string(),
                "操作工作流实例错误".to_string(),
            ),
            Error::CheckProcessDefinitionError => AuroraErrorInfo::new(
                50016,
                "check process definition error".to_string(),
                "工作流定义错误".to_string(),
            ),
            Error::QueryRecipientsAndCopyersByProcessDefinitionError => AuroraErrorInfo::new(
                50017,
                "query recipients and copyers by process definition error".to_string(),
                "查询收件人和抄送人错误".to_string(),
            ),
            Error::DataIsNotValid => AuroraErrorInfo::new(
                50017,
                "data {0} not valid".to_string(),
                "数据[{0}]无效".to_string(),
            ),
            Error::DataIsNull => AuroraErrorInfo::new(
                50018,
                "data {0} is null".to_string(),
                "数据[{0}]不能为空".to_string(),
            ),
            Error::ProcessNodeHasCycle => AuroraErrorInfo::new(
                50019,
                "process node has cycle".to_string(),
                "流程节点间存在循环依赖".to_string(),
            ),
            Error::ProcessNodeSParameterInvalid => AuroraErrorInfo::new(
                50020,
                "process node {0} parameter invalid".to_string(),
                "流程节点[{0}]参数无效".to_string(),
            ),
            Error::ProcessDefineStateOnline => AuroraErrorInfo::new(
                50021,
                "process definition [{0}] is already online".to_string(),
                "工作流定义[{0}]已上线".to_string(),
            ),
            Error::DeleteProcessDefineByCodeError => AuroraErrorInfo::new(
                50022,
                "delete process definition by code error".to_string(),
                "删除工作流定义错误".to_string(),
            ),
            Error::ScheduleCronStateOnline => AuroraErrorInfo::new(
                50023,
                "the status of schedule {0} is already online".to_string(),
                "调度配置[{0}]已上线".to_string(),
            ),
            Error::DeleteScheduleCronByIdError => AuroraErrorInfo::new(
                50024,
                "delete schedule by id error".to_string(),
                "删除调度配置错误".to_string(),
            ),
            Error::BatchDeleteProcessDefineError => AuroraErrorInfo::new(
                50025,
                "batch delete process definition error".to_string(),
                "批量删除工作流定义错误".to_string(),
            ),
            Error::BatchDeleteProcessDefineByCodesError => AuroraErrorInfo::new(
                50026,
                "batch delete process definition by codes {0} error".to_string(),
                "批量删除工作流定义[{0}]错误".to_string(),
            ),
            Error::DeleteProcessDefineByCodesError => AuroraErrorInfo::new(
                50026,
                "delete process definition by codes {0} error".to_string(),
                "删除工作流定义[{0}]错误".to_string(),
            ),
            Error::TenantNotSuitable => AuroraErrorInfo::new(
                50027,
                "there is not any tenant suitable: please choose a tenant available.".to_string(),
                "没有合适的租户，请选择可用的租户".to_string(),
            ),
            Error::ExportProcessDefineByIdError => AuroraErrorInfo::new(
                50028,
                "export process definition by id error".to_string(),
                "导出工作流定义错误".to_string(),
            ),
            Error::BatchExportProcessDefineByIdsError => AuroraErrorInfo::new(
                50028,
                "batch export process definition by ids error".to_string(),
                "批量导出工作流定义错误".to_string(),
            ),
            Error::ImportProcessDefineError => AuroraErrorInfo::new(
                50029,
                "import process definition error".to_string(),
                "导入工作流定义错误".to_string(),
            ),
            Error::TaskDefineNotExist => AuroraErrorInfo::new(
                50030,
                "task definition [{0}] does not exist".to_string(),
                "任务定义[{0}]不存在".to_string(),
            ),
            Error::CreateProcessTaskRelationError => AuroraErrorInfo::new(
                50032,
                "create process task relation error".to_string(),
                "创建工作流任务关系错误".to_string(),
            ),
            Error::ProcessTaskRelationNotExist => AuroraErrorInfo::new(
                50033,
                "process task relation [{0}] does not exist".to_string(),
                "工作流任务关系[{0}]不存在".to_string(),
            ),
            Error::ProcessTaskRelationExist => AuroraErrorInfo::new(
                50034,
                "process task relation is already exist  processCode:[{0}]".to_string(),
                "工作流任务关系已存在".to_string(),
            ),
            Error::ProcessDagIsEmpty => AuroraErrorInfo::new(
                50035,
                "process dag is empty".to_string(),
                "工作流dag是空".to_string(),
            ),
            Error::CheckProcessTaskRelationError => AuroraErrorInfo::new(
                50036,
                "check process task relation error".to_string(),
                "工作流任务关系参数错误".to_string(),
            ),
            Error::CreateTaskDefinitionError => AuroraErrorInfo::new(
                50037,
                "create task definition error".to_string(),
                "创建任务错误".to_string(),
            ),
            Error::UpdateTaskDefinitionError => AuroraErrorInfo::new(
                50038,
                "update task definition error".to_string(),
                "更新任务定义错误".to_string(),
            ),
            Error::QueryTaskDefinitionVersionsError => AuroraErrorInfo::new(
                50039,
                "query task definition versions error".to_string(),
                "查询任务历史版本信息出错".to_string(),
            ),
            Error::SwitchTaskDefinitionVersionError => AuroraErrorInfo::new(
                50040,
                "Switch task definition version error".to_string(),
                "切换任务版本出错".to_string(),
            ),
            Error::DeleteTaskDefinitionVersionError => AuroraErrorInfo::new(
                50041,
                "delete task definition version error".to_string(),
                "删除任务历史版本出错".to_string(),
            ),
            Error::DeleteTaskDefineByCodeError => AuroraErrorInfo::new(
                50042,
                "delete task definition by code error".to_string(),
                "删除任务定义错误".to_string(),
            ),
            Error::QueryDetailOfTaskDefinitionError => AuroraErrorInfo::new(
                50043,
                "query detail of task definition error".to_string(),
                "查询任务详细信息错误".to_string(),
            ),
            Error::QueryTaskDefinitionListPagingError => AuroraErrorInfo::new(
                50044,
                "query task definition list paging error".to_string(),
                "分页查询任务定义列表错误".to_string(),
            ),
            Error::TaskDefinitionNameExisted => AuroraErrorInfo::new(
                50045,
                "task definition name [{0}] already exists".to_string(),
                "任务定义名称[{0}]已经存在".to_string(),
            ),
            Error::ReleaseTaskDefinitionError => AuroraErrorInfo::new(
                50046,
                "release task definition error".to_string(),
                "上线任务错误".to_string(),
            ),
            Error::MoveProcessTaskRelationError => AuroraErrorInfo::new(
                50047,
                "move process task relation error".to_string(),
                "移动任务到其他工作流错误".to_string(),
            ),
            Error::DeleteTaskProcessRelationError => AuroraErrorInfo::new(
                50048,
                "delete process task relation error".to_string(),
                "删除工作流任务关系错误".to_string(),
            ),
            Error::QueryTaskProcessRelationError => AuroraErrorInfo::new(
                50049,
                "query process task relation error".to_string(),
                "查询工作流任务关系错误".to_string(),
            ),
            Error::TaskDefineStateOnline => AuroraErrorInfo::new(
                50050,
                "task definition [{0}] is already online".to_string(),
                "任务定义[{0}]已上线".to_string(),
            ),
            Error::TaskHasDownstream => AuroraErrorInfo::new(
                50051,
                "Task exists downstream [{0}] dependence".to_string(),
                "任务存在下游[{0}]依赖".to_string(),
            ),
            Error::TaskHasUpstream => AuroraErrorInfo::new(
                50052,
                "Task [{0}] exists upstream dependence".to_string(),
                "任务[{0}]存在上游依赖".to_string(),
            ),
            Error::MainTableUsingVersion => AuroraErrorInfo::new(
                50053,
                "the version that the master table is using".to_string(),
                "主表正在使用该版本".to_string(),
            ),
            Error::ProjectProcessNotMatch => AuroraErrorInfo::new(
                50054,
                "the project and the process is not match".to_string(),
                "项目和工作流不匹配".to_string(),
            ),
            Error::DeleteEdgeError => AuroraErrorInfo::new(
                50055,
                "delete edge error".to_string(),
                "删除工作流任务连接线错误".to_string(),
            ),
            Error::NotSupportUpdateTaskDefinition => AuroraErrorInfo::new(
                50056,
                "task state does not support modification".to_string(),
                "当前任务不支持修改".to_string(),
            ),
            Error::NotSupportCopyTaskType => AuroraErrorInfo::new(
                50057,
                "task type [{0}] does not support copy".to_string(),
                "不支持复制的任务类型[{0}]".to_string(),
            ),
            Error::HdfsNotStartup => AuroraErrorInfo::new(
                60001,
                "hdfs not startup".to_string(),
                "hdfs未启用".to_string(),
            ),
            Error::StorageNotStartup => AuroraErrorInfo::new(
                60002,
                "storage not startup".to_string(),
                "存储未启用".to_string(),
            ),
            Error::S3CannotRename => AuroraErrorInfo::new(
                60003,
                "directory cannot be renamed".to_string(),
                "S3无法重命名文件夹".to_string(),
            ),
            Error::QueryDatabaseStateError => AuroraErrorInfo::new(
                70001,
                "query database state error".to_string(),
                "查询数据库状态错误".to_string(),
            ),
            Error::CreateAccessTokenError => AuroraErrorInfo::new(
                70010,
                "create access token error".to_string(),
                "创建访问token错误".to_string(),
            ),
            Error::GenerateTokenError => AuroraErrorInfo::new(
                70011,
                "generate token error".to_string(),
                "生成token错误".to_string(),
            ),
            Error::QueryAccesstokenListPagingError => AuroraErrorInfo::new(
                70012,
                "query access token list paging error".to_string(),
                "分页查询访问token列表错误".to_string(),
            ),
            Error::UpdateAccessTokenError => AuroraErrorInfo::new(
                70013,
                "update access token error".to_string(),
                "更新访问token错误".to_string(),
            ),
            Error::DeleteAccessTokenError => AuroraErrorInfo::new(
                70014,
                "delete access token error".to_string(),
                "删除访问token错误".to_string(),
            ),
            Error::AccessTokenNotExist => AuroraErrorInfo::new(
                70015,
                "access token not exist".to_string(),
                "访问token不存在".to_string(),
            ),
            Error::QueryAccesstokenByUserError => AuroraErrorInfo::new(
                70016,
                "query access token by user error".to_string(),
                "查询访问指定用户的token错误".to_string(),
            ),
            Error::CommandStateCountError => AuroraErrorInfo::new(
                80001,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            ),
            Error::NegativeSizeNumberError => AuroraErrorInfo::new(
                80002,
                "query size number error".to_string(),
                "查询size错误".to_string(),
            ),
            Error::StartTimeBiggerThanEndTimeError => AuroraErrorInfo::new(
                80003,
                "start time bigger than end time error".to_string(),
                "开始时间在结束时间之后错误".to_string(),
            ),
            Error::QueueCountError => AuroraErrorInfo::new(
                90001,
                "queue count error".to_string(),
                "查询队列数据错误".to_string(),
            ),
            Error::KerberosStartupState => AuroraErrorInfo::new(
                100001,
                "get kerberos startup state error".to_string(),
                "获取kerberos启动状态错误".to_string(),
            ),
            Error::QueryAuditLogListPaging => AuroraErrorInfo::new(
                10057,
                "query audit log list paging".to_string(),
                "分页查询日志列表错误".to_string(),
            ),
            Error::PluginNotAUiComponent => AuroraErrorInfo::new(
                110001,
                "query plugin error: this plugin has no UI component".to_string(),
                "查询插件错误，此插件无UI组件".to_string(),
            ),
            Error::QueryPluginsResultIsNull => AuroraErrorInfo::new(
                110002,
                "query alarm plugins result is empty:please check the startup status of the alarm \
                 component and confirm that the relevant alarm plugin is successfully registered"
                    .to_string(),
                "查询告警插件为空".to_string(),
            ),
            Error::QueryPluginsError => AuroraErrorInfo::new(
                110003,
                "query plugins error".to_string(),
                "查询插件错误".to_string(),
            ),
            Error::QueryPluginDetailResultIsNull => AuroraErrorInfo::new(
                110004,
                "query plugin detail result is null".to_string(),
                "查询插件详情结果为空".to_string(),
            ),
            Error::UpdateAlertPluginInstanceError => AuroraErrorInfo::new(
                110005,
                "update alert plugin instance error".to_string(),
                "更新告警组和告警组插件实例错误".to_string(),
            ),
            Error::DeleteAlertPluginInstanceError => AuroraErrorInfo::new(
                110006,
                "delete alert plugin instance error".to_string(),
                "删除告警组和告警组插件实例错误".to_string(),
            ),
            Error::GetAlertPluginInstanceError => AuroraErrorInfo::new(
                110007,
                "get alert plugin instance error".to_string(),
                "获取告警组和告警组插件实例错误".to_string(),
            ),
            Error::CreateAlertPluginInstanceError => AuroraErrorInfo::new(
                110008,
                "create alert plugin instance error".to_string(),
                "创建告警组和告警组插件实例错误".to_string(),
            ),
            Error::QueryAllAlertPluginInstanceError => AuroraErrorInfo::new(
                110009,
                "query all alert plugin instance error".to_string(),
                "查询所有告警实例失败".to_string(),
            ),
            Error::PluginInstanceAlreadyExit => AuroraErrorInfo::new(
                110010,
                "plugin instance already exit".to_string(),
                "该告警插件实例已存在".to_string(),
            ),
            Error::ListPagingAlertPluginInstanceError => AuroraErrorInfo::new(
                110011,
                "query plugin instance page error".to_string(),
                "分页查询告警实例失败".to_string(),
            ),
            Error::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated => AuroraErrorInfo::new(
                110012,
                "failed to delete the alert instance there is an alarm group associated with this \
                 alert instance"
                    .to_string(),
                "删除告警实例失败，存在与此告警实例关联的警报组".to_string(),
            ),
            Error::ProcessDefinitionVersionIsUsed => AuroraErrorInfo::new(
                110013,
                "this process definition version is used".to_string(),
                "此工作流定义版本被使用".to_string(),
            ),
            Error::CreateEnvironmentError => AuroraErrorInfo::new(
                120001,
                "create environment error".to_string(),
                "创建环境失败".to_string(),
            ),
            Error::EnvironmentNameExists => AuroraErrorInfo::new(
                120002,
                "this environment name [{0}] already exists".to_string(),
                "环境名称[{0}]已经存在".to_string(),
            ),
            Error::EnvironmentNameIsNull => AuroraErrorInfo::new(
                120003,
                "this environment name shouldn't be empty.".to_string(),
                "环境名称不能为空".to_string(),
            ),
            Error::EnvironmentConfigIsNull => AuroraErrorInfo::new(
                120004,
                "this environment config shouldn't be empty.".to_string(),
                "环境配置信息不能为空".to_string(),
            ),
            Error::UpdateEnvironmentError => AuroraErrorInfo::new(
                120005,
                "update environment [{0}] info error".to_string(),
                "更新环境[{0}]信息失败".to_string(),
            ),
            Error::DeleteEnvironmentError => AuroraErrorInfo::new(
                120006,
                "delete environment error".to_string(),
                "删除环境信息失败".to_string(),
            ),
            Error::DeleteEnvironmentRelatedTaskExists => AuroraErrorInfo::new(
                120007,
                "delete environment error, related task exists".to_string(),
                "删除环境信息失败，存在关联任务".to_string(),
            ),
            Error::QueryEnvironmentByNameError => AuroraErrorInfo::new(
                1200008,
                "not found environment [{0}] ".to_string(),
                "查询环境名称[{0}]信息不存在".to_string(),
            ),
            Error::QueryEnvironmentByCodeError => AuroraErrorInfo::new(
                1200009,
                "not found environment [{0}] ".to_string(),
                "查询环境编码[{0}]不存在".to_string(),
            ),
            Error::QueryEnvironmentError => AuroraErrorInfo::new(
                1200010,
                "login user query environment error".to_string(),
                "分页查询环境列表错误".to_string(),
            ),
            Error::VerifyEnvironmentError => AuroraErrorInfo::new(
                1200011,
                "verify environment error".to_string(),
                "验证环境信息错误".to_string(),
            ),
            Error::GetRuleFormCreateJsonError => AuroraErrorInfo::new(
                1200012,
                "get rule form create json error".to_string(),
                "获取规则 FROM-CREATE-JSON 错误".to_string(),
            ),
            Error::QueryRuleListPagingError => AuroraErrorInfo::new(
                1200013,
                "query rule list paging error".to_string(),
                "获取规则分页列表错误".to_string(),
            ),
            Error::QueryRuleListError => AuroraErrorInfo::new(
                1200014,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            ),
            Error::QueryRuleInputEntryListError => AuroraErrorInfo::new(
                1200015,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            ),
            Error::QueryExecuteResultListPagingError => AuroraErrorInfo::new(
                1200016,
                "query execute result list paging error".to_string(),
                "获取数据质量任务结果分页错误".to_string(),
            ),
            Error::GetDatasourceOptionsError => AuroraErrorInfo::new(
                1200017,
                "get datasource options error".to_string(),
                "获取数据源Options错误".to_string(),
            ),
            Error::GetDatasourceTablesError => AuroraErrorInfo::new(
                1200018,
                "get datasource tables error".to_string(),
                "获取数据源表列表错误".to_string(),
            ),
            Error::GetDatasourceTableColumnsError => AuroraErrorInfo::new(
                1200019,
                "get datasource table columns error".to_string(),
                "获取数据源表列名错误".to_string(),
            ),
            Error::TaskGroupNameExist => AuroraErrorInfo::new(
                130001,
                "this task group name is repeated in a project".to_string(),
                "该任务组名称在一个项目中已经使用".to_string(),
            ),
            Error::TaskGroupSizeError => AuroraErrorInfo::new(
                130002,
                "task group size error".to_string(),
                "任务组大小应该为大于1的整数".to_string(),
            ),
            Error::TaskGroupStatusError => AuroraErrorInfo::new(
                130003,
                "task group status error".to_string(),
                "任务组已经被关闭".to_string(),
            ),
            Error::TaskGroupFull => AuroraErrorInfo::new(
                130004,
                "task group is full".to_string(),
                "任务组已经满了".to_string(),
            ),
            Error::TaskGroupUsedSizeError => AuroraErrorInfo::new(
                130005,
                "the used size number of task group is dirty".to_string(),
                "任务组使用的容量发生了变化".to_string(),
            ),
            Error::TaskGroupQueueReleaseError => AuroraErrorInfo::new(
                130006,
                "failed to release task group queue".to_string(),
                "任务组资源释放时出现了错误".to_string(),
            ),
            Error::TaskGroupQueueAwakeError => AuroraErrorInfo::new(
                130007,
                "awake waiting task failed".to_string(),
                "任务组使唤醒等待任务时发生了错误".to_string(),
            ),
            Error::CreateTaskGroupError => AuroraErrorInfo::new(
                130008,
                "create task group error".to_string(),
                "创建任务组错误".to_string(),
            ),
            Error::UpdateTaskGroupError => AuroraErrorInfo::new(
                130009,
                "update task group list error".to_string(),
                "更新任务组错误".to_string(),
            ),
            Error::QueryTaskGroupListError => AuroraErrorInfo::new(
                130010,
                "query task group list error".to_string(),
                "查询任务组列表错误".to_string(),
            ),
            Error::CloseTaskGroupError => AuroraErrorInfo::new(
                130011,
                "close task group error".to_string(),
                "关闭任务组错误".to_string(),
            ),
            Error::StartTaskGroupError => AuroraErrorInfo::new(
                130012,
                "start task group error".to_string(),
                "启动任务组错误".to_string(),
            ),
            Error::QueryTaskGroupQueueListError => AuroraErrorInfo::new(
                130013,
                "query task group queue list error".to_string(),
                "查询任务组队列列表错误".to_string(),
            ),
            Error::TaskGroupCacheStartFailed => AuroraErrorInfo::new(
                130014,
                "cache start failed".to_string(),
                "任务组相关的缓存启动失败".to_string(),
            ),
            Error::EnvironmentWorkerGroupsIsInvalid => AuroraErrorInfo::new(
                130015,
                "environment worker groups is invalid format".to_string(),
                "环境关联的工作组参数解析错误".to_string(),
            ),
            Error::UpdateEnvironmentWorkerGroupRelationError => AuroraErrorInfo::new(
                130016,
                "update environment worker group relation error".to_string(),
                "更新环境关联的工作组错误".to_string(),
            ),
            Error::TaskGroupQueueAlreadyStart => AuroraErrorInfo::new(
                130017,
                "task group queue already start".to_string(),
                "节点已经获取任务组资源".to_string(),
            ),
            Error::TaskGroupStatusClosed => AuroraErrorInfo::new(
                130018,
                "The task group has been closed.".to_string(),
                "任务组已经被关闭".to_string(),
            ),
            Error::TaskGroupStatusOpened => AuroraErrorInfo::new(
                130019,
                "The task group has been opened.".to_string(),
                "任务组已经被开启".to_string(),
            ),
            Error::NotAllowToDisableOwnAccount => AuroraErrorInfo::new(
                130020,
                "Not allow to disable your own account".to_string(),
                "不能停用自己的账号".to_string(),
            ),
            Error::NotAllowToDeleteDefaultAlarmGroup => AuroraErrorInfo::new(
                130030,
                "Not allow to delete the default alarm group ".to_string(),
                "不能删除默认告警组".to_string(),
            ),
            Error::TimeZoneIllegal => AuroraErrorInfo::new(
                130031,
                "time zone [{0}] is illegal".to_string(),
                "时区参数 [{0}] 不合法".to_string(),
            ),
            Error::QueryK8sNamespaceListPagingError => AuroraErrorInfo::new(
                1300001,
                "login user query k8s namespace list paging error".to_string(),
                "分页查询k8s名称空间列表错误".to_string(),
            ),
            Error::K8sNamespaceExist => AuroraErrorInfo::new(
                1300002,
                "k8s namespace {0} already exists".to_string(),
                "k8s命名空间[{0}]已存在".to_string(),
            ),
            Error::CreateK8sNamespaceError => AuroraErrorInfo::new(
                1300003,
                "create k8s namespace error".to_string(),
                "创建k8s命名空间错误".to_string(),
            ),
            Error::UpdateK8sNamespaceError => AuroraErrorInfo::new(
                1300004,
                "update k8s namespace error".to_string(),
                "更新k8s命名空间信息错误".to_string(),
            ),
            Error::K8sNamespaceNotExist => AuroraErrorInfo::new(
                1300005,
                "k8s namespace {0} not exists".to_string(),
                "命名空间ID[{0}]不存在".to_string(),
            ),
            Error::K8sClientOpsError => AuroraErrorInfo::new(
                1300006,
                "k8s error with exception {0}".to_string(),
                "k8s操作报错[{0}]".to_string(),
            ),
            Error::VerifyK8sNamespaceError => AuroraErrorInfo::new(
                1300007,
                "verify k8s and namespace error".to_string(),
                "验证k8s命名空间信息错误".to_string(),
            ),
            Error::DeleteK8sNamespaceByIdError => AuroraErrorInfo::new(
                1300008,
                "delete k8s namespace by id error".to_string(),
                "删除命名空间错误".to_string(),
            ),
            Error::VerifyParameterNameFailed => AuroraErrorInfo::new(
                1300009,
                "The file name verify failed".to_string(),
                "文件命名校验失败".to_string(),
            ),
            Error::StoreOperateCreateError => AuroraErrorInfo::new(
                1300010,
                "create the resource failed".to_string(),
                "存储操作失败".to_string(),
            ),
            Error::GrantK8sNamespaceError => AuroraErrorInfo::new(
                1300011,
                "grant namespace error".to_string(),
                "授权资源错误".to_string(),
            ),
            Error::QueryUnauthorizedNamespaceError => AuroraErrorInfo::new(
                1300012,
                "query unauthorized namespace error".to_string(),
                "查询未授权命名空间错误".to_string(),
            ),
            Error::QueryAuthorizedNamespaceError => AuroraErrorInfo::new(
                1300013,
                "query authorized namespace error".to_string(),
                "查询授权命名空间错误".to_string(),
            ),
            Error::QueryCanUseK8sClusterError => AuroraErrorInfo::new(
                1300014,
                "login user query can used k8s cluster list error".to_string(),
                "查询可用k8s集群错误".to_string(),
            ),
            Error::ResourceFullNameTooLongError => AuroraErrorInfo::new(
                1300015,
                "resource's fullname is too long error".to_string(),
                "资源文件名过长".to_string(),
            ),
            Error::TenantFullNameTooLongError => AuroraErrorInfo::new(
                1300016,
                "tenant's fullname is too long error".to_string(),
                "租户名过长".to_string(),
            ),
        }
    }
}
