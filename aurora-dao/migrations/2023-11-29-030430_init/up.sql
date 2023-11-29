-- Your SQL goes here
ALTER TABLE
    IF EXISTS ONLY public.t_ds_worker_group DROP CONSTRAINT IF EXISTS t_ds_worker_group_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_version DROP CONSTRAINT IF EXISTS t_ds_version_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_user DROP CONSTRAINT IF EXISTS t_ds_user_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_udfs DROP CONSTRAINT IF EXISTS t_ds_udfs_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_trigger_relation DROP CONSTRAINT IF EXISTS t_ds_trigger_relation_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_trigger_relation DROP CONSTRAINT IF EXISTS t_ds_trigger_relation_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_tenant DROP CONSTRAINT IF EXISTS t_ds_tenant_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_task_instance DROP CONSTRAINT IF EXISTS t_ds_task_instance_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_task_group_queue DROP CONSTRAINT IF EXISTS t_ds_task_group_queue_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_task_group DROP CONSTRAINT IF EXISTS t_ds_task_group_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_task_definition DROP CONSTRAINT IF EXISTS t_ds_task_definition_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_task_definition_log DROP CONSTRAINT IF EXISTS t_ds_task_definition_log_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_session DROP CONSTRAINT IF EXISTS t_ds_session_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_schedules DROP CONSTRAINT IF EXISTS t_ds_schedules_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_resources DROP CONSTRAINT IF EXISTS t_ds_resources_un;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_resources DROP CONSTRAINT IF EXISTS t_ds_resources_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_udfs_user DROP CONSTRAINT IF EXISTS t_ds_relation_udfs_user_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_sub_workflow DROP CONSTRAINT IF EXISTS t_ds_relation_sub_workflow_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_rule_input_entry DROP CONSTRAINT IF EXISTS t_ds_relation_rule_input_entry_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_rule_execute_sql DROP CONSTRAINT IF EXISTS t_ds_relation_rule_execute_sql_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_resources_user DROP CONSTRAINT IF EXISTS t_ds_relation_resources_user_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_project_user DROP CONSTRAINT IF EXISTS t_ds_relation_project_user_un;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_project_user DROP CONSTRAINT IF EXISTS t_ds_relation_project_user_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_process_instance DROP CONSTRAINT IF EXISTS t_ds_relation_process_instance_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_namespace_user DROP CONSTRAINT IF EXISTS t_ds_relation_namespace_user_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_datasource_user DROP CONSTRAINT IF EXISTS t_ds_relation_datasource_user_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_queue DROP CONSTRAINT IF EXISTS t_ds_queue_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_project_preference DROP CONSTRAINT IF EXISTS t_ds_project_preference_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_project DROP CONSTRAINT IF EXISTS t_ds_project_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_project_parameter DROP CONSTRAINT IF EXISTS t_ds_project_parameter_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_process_task_relation DROP CONSTRAINT IF EXISTS t_ds_process_task_relation_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_process_task_relation_log DROP CONSTRAINT IF EXISTS t_ds_process_task_relation_log_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_process_instance DROP CONSTRAINT IF EXISTS t_ds_process_instance_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_process_definition DROP CONSTRAINT IF EXISTS t_ds_process_definition_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_process_definition_log DROP CONSTRAINT IF EXISTS t_ds_process_definition_log_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_plugin_define DROP CONSTRAINT IF EXISTS t_ds_plugin_define_un;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_plugin_define DROP CONSTRAINT IF EXISTS t_ds_plugin_define_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_k8s DROP CONSTRAINT IF EXISTS t_ds_k8s_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_k8s_namespace DROP CONSTRAINT IF EXISTS t_ds_k8s_namespace_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_fav_task DROP CONSTRAINT IF EXISTS t_ds_fav_task_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_error_command DROP CONSTRAINT IF EXISTS t_ds_error_command_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment_worker_group_relation DROP CONSTRAINT IF EXISTS t_ds_environment_worker_group_relation_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment DROP CONSTRAINT IF EXISTS t_ds_environment_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_dq_task_statistics_value DROP CONSTRAINT IF EXISTS t_ds_dq_task_statistics_value_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_dq_rule DROP CONSTRAINT IF EXISTS t_ds_dq_rule_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_dq_rule_input_entry DROP CONSTRAINT IF EXISTS t_ds_dq_rule_input_entry_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_dq_rule_execute_sql DROP CONSTRAINT IF EXISTS t_ds_dq_rule_execute_sql_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_dq_execute_result DROP CONSTRAINT IF EXISTS t_ds_dq_execute_result_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_dq_comparison_type DROP CONSTRAINT IF EXISTS t_ds_dq_comparison_type_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_datasource DROP CONSTRAINT IF EXISTS t_ds_datasource_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_datasource DROP CONSTRAINT IF EXISTS t_ds_datasource_name_un;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_command DROP CONSTRAINT IF EXISTS t_ds_command_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_cluster DROP CONSTRAINT IF EXISTS t_ds_cluster_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_audit_log DROP CONSTRAINT IF EXISTS t_ds_audit_log_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_alertgroup DROP CONSTRAINT IF EXISTS t_ds_alertgroup_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_alertgroup DROP CONSTRAINT IF EXISTS t_ds_alertgroup_name_un;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_alert_send_status DROP CONSTRAINT IF EXISTS t_ds_alert_send_status_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_alert_plugin_instance DROP CONSTRAINT IF EXISTS t_ds_alert_plugin_instance_pk;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_alert DROP CONSTRAINT IF EXISTS t_ds_alert_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_access_token DROP CONSTRAINT IF EXISTS t_ds_access_token_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_triggers DROP CONSTRAINT IF EXISTS qrtz_triggers_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_simprop_triggers DROP CONSTRAINT IF EXISTS qrtz_simprop_triggers_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_simple_triggers DROP CONSTRAINT IF EXISTS qrtz_simple_triggers_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_scheduler_state DROP CONSTRAINT IF EXISTS qrtz_scheduler_state_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_paused_trigger_grps DROP CONSTRAINT IF EXISTS qrtz_paused_trigger_grps_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_locks DROP CONSTRAINT IF EXISTS qrtz_locks_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_job_details DROP CONSTRAINT IF EXISTS qrtz_job_details_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_fired_triggers DROP CONSTRAINT IF EXISTS qrtz_fired_triggers_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_cron_triggers DROP CONSTRAINT IF EXISTS qrtz_cron_triggers_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_calendars DROP CONSTRAINT IF EXISTS qrtz_calendars_pkey;

ALTER TABLE
    IF EXISTS ONLY public.qrtz_blob_triggers DROP CONSTRAINT IF EXISTS qrtz_blob_triggers_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_process_definition DROP CONSTRAINT IF EXISTS process_definition_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_relation_namespace_user DROP CONSTRAINT IF EXISTS namespace_user_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_worker_group DROP CONSTRAINT IF EXISTS name_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_k8s_namespace DROP CONSTRAINT IF EXISTS k8s_namespace_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment_worker_group_relation DROP CONSTRAINT IF EXISTS environment_worker_group_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment DROP CONSTRAINT IF EXISTS environment_name_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment DROP CONSTRAINT IF EXISTS environment_code_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_cluster DROP CONSTRAINT IF EXISTS cluster_name_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_cluster DROP CONSTRAINT IF EXISTS cluster_code_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_alert_send_status DROP CONSTRAINT IF EXISTS alert_send_status_unique;

ALTER TABLE
    IF EXISTS public.t_ds_trigger_relation
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_task_group_queue
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_task_group
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_relation_sub_workflow
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_relation_rule_input_entry
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_relation_rule_execute_sql
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_relation_namespace_user
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_plugin_define
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_k8s_namespace
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_k8s
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_fav_task
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_environment_worker_group_relation
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_environment
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_dq_task_statistics_value
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_dq_rule_input_entry
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_dq_rule_execute_sql
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_dq_rule
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_dq_execute_result
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_dq_comparison_type
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_cluster
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_audit_log
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_alert_send_status
ALTER COLUMN
    id DROP DEFAULT;

ALTER TABLE
    IF EXISTS public.t_ds_alert_plugin_instance
ALTER COLUMN
    id DROP DEFAULT;

-- *not* dropping schema, since initdb creates it
--
-- Name: public; Type: SCHEMA; Schema: -; Owner: root
--
-- *not* creating schema, since initdb creates it
--
-- Name: qrtz_blob_triggers; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_blob_triggers (
    sched_name character varying(120) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    blob_data bytea
);

ALTER TABLE
    public.qrtz_blob_triggers OWNER TO root;

--
-- Name: qrtz_calendars; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_calendars (
    sched_name character varying(120) NOT NULL,
    calendar_name character varying(200) NOT NULL,
    calendar bytea NOT NULL
);

ALTER TABLE
    public.qrtz_calendars OWNER TO root;

--
-- Name: qrtz_cron_triggers; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_cron_triggers (
    sched_name character varying(120) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    cron_expression character varying(120) NOT NULL,
    time_zone_id character varying(80)
);

ALTER TABLE
    public.qrtz_cron_triggers OWNER TO root;

--
-- Name: qrtz_fired_triggers; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_fired_triggers (
    sched_name character varying(120) NOT NULL,
    entry_id character varying(200) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    instance_name character varying(200) NOT NULL,
    fired_time bigint NOT NULL,
    sched_time bigint NOT NULL,
    priority integer NOT NULL,
    state character varying(16) NOT NULL,
    job_name character varying(200),
    job_group character varying(200),
    is_nonconcurrent boolean,
    requests_recovery boolean
);

ALTER TABLE
    public.qrtz_fired_triggers OWNER TO root;

--
-- Name: qrtz_job_details; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_job_details (
    sched_name character varying(120) NOT NULL,
    job_name character varying(200) NOT NULL,
    job_group character varying(200) NOT NULL,
    description character varying(250),
    job_class_name character varying(250) NOT NULL,
    is_durable boolean NOT NULL,
    is_nonconcurrent boolean NOT NULL,
    is_update_data boolean NOT NULL,
    requests_recovery boolean NOT NULL,
    job_data bytea
);

ALTER TABLE
    public.qrtz_job_details OWNER TO root;

--
-- Name: qrtz_locks; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_locks (
    sched_name character varying(120) NOT NULL,
    lock_name character varying(40) NOT NULL
);

ALTER TABLE
    public.qrtz_locks OWNER TO root;

--
-- Name: qrtz_paused_trigger_grps; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_paused_trigger_grps (
    sched_name character varying(120) NOT NULL,
    trigger_group character varying(200) NOT NULL
);

ALTER TABLE
    public.qrtz_paused_trigger_grps OWNER TO root;

--
-- Name: qrtz_scheduler_state; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_scheduler_state (
    sched_name character varying(120) NOT NULL,
    instance_name character varying(200) NOT NULL,
    last_checkin_time bigint NOT NULL,
    checkin_interval bigint NOT NULL
);

ALTER TABLE
    public.qrtz_scheduler_state OWNER TO root;

--
-- Name: qrtz_simple_triggers; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_simple_triggers (
    sched_name character varying(120) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    repeat_count bigint NOT NULL,
    repeat_interval bigint NOT NULL,
    times_triggered bigint NOT NULL
);

ALTER TABLE
    public.qrtz_simple_triggers OWNER TO root;

--
-- Name: qrtz_simprop_triggers; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_simprop_triggers (
    sched_name character varying(120) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    str_prop_1 character varying(512),
    str_prop_2 character varying(512),
    str_prop_3 character varying(512),
    int_prop_1 integer,
    int_prop_2 integer,
    long_prop_1 bigint,
    long_prop_2 bigint,
    dec_prop_1 numeric(13, 4),
    dec_prop_2 numeric(13, 4),
    bool_prop_1 boolean,
    bool_prop_2 boolean
);

ALTER TABLE
    public.qrtz_simprop_triggers OWNER TO root;

--
-- Name: qrtz_triggers; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.qrtz_triggers (
    sched_name character varying(120) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    job_name character varying(200) NOT NULL,
    job_group character varying(200) NOT NULL,
    description character varying(250),
    next_fire_time bigint,
    prev_fire_time bigint,
    priority integer,
    trigger_state character varying(16) NOT NULL,
    trigger_type character varying(8) NOT NULL,
    start_time bigint NOT NULL,
    end_time bigint,
    calendar_name character varying(200),
    misfire_instr smallint,
    job_data bytea
);

ALTER TABLE
    public.qrtz_triggers OWNER TO root;

--
-- Name: t_ds_access_token_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_access_token_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_access_token_id_sequence OWNER TO root;

--
-- Name: t_ds_access_token; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_access_token (
    id integer DEFAULT nextval(
        'public.t_ds_access_token_id_sequence' :: regclass
    ) NOT NULL,
    user_id integer,
    token character varying(64) DEFAULT NULL :: character varying,
    expire_time timestamp without time zone,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_access_token OWNER TO root;

--
-- Name: t_ds_alert_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_alert_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_alert_id_sequence OWNER TO root;

--
-- Name: t_ds_alert; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_alert (
    id integer DEFAULT nextval('public.t_ds_alert_id_sequence' :: regclass) NOT NULL,
    title character varying(512) DEFAULT NULL :: character varying,
    sign character varying(40) DEFAULT '' :: character varying NOT NULL,
    content text,
    alert_status integer DEFAULT 0,
    warning_type integer DEFAULT 2,
    log text,
    alertgroup_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    project_code bigint,
    process_definition_code bigint,
    process_instance_id integer,
    alert_type integer
);

ALTER TABLE
    public.t_ds_alert OWNER TO root;

--
-- Name: COLUMN t_ds_alert.sign; Type: COMMENT; Schema: public; Owner: root
--
COMMENT ON COLUMN public.t_ds_alert.sign IS 'sign=sha1(content)';

--
-- Name: t_ds_alert_plugin_instance; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_alert_plugin_instance (
    id integer NOT NULL,
    plugin_define_id integer NOT NULL,
    plugin_instance_params text,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    instance_name character varying(255)
);

ALTER TABLE
    public.t_ds_alert_plugin_instance OWNER TO root;

--
-- Name: t_ds_alert_plugin_instance_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_alert_plugin_instance_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_alert_plugin_instance_id_seq OWNER TO root;

--
-- Name: t_ds_alert_plugin_instance_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_alert_plugin_instance_id_seq OWNED BY public.t_ds_alert_plugin_instance.id;

--
-- Name: t_ds_alert_send_status; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_alert_send_status (
    id integer NOT NULL,
    alert_id integer NOT NULL,
    alert_plugin_instance_id integer NOT NULL,
    send_status integer DEFAULT 0,
    log text,
    create_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_alert_send_status OWNER TO root;

--
-- Name: t_ds_alert_send_status_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_alert_send_status_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_alert_send_status_id_seq OWNER TO root;

--
-- Name: t_ds_alert_send_status_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_alert_send_status_id_seq OWNED BY public.t_ds_alert_send_status.id;

--
-- Name: t_ds_alertgroup_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_alertgroup_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_alertgroup_id_sequence OWNER TO root;

--
-- Name: t_ds_alertgroup; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_alertgroup (
    id integer DEFAULT nextval('public.t_ds_alertgroup_id_sequence' :: regclass) NOT NULL,
    alert_instance_ids character varying(255) DEFAULT NULL :: character varying,
    create_user_id integer,
    group_name character varying(255) DEFAULT NULL :: character varying,
    description character varying(255) DEFAULT NULL :: character varying,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_alertgroup OWNER TO root;

--
-- Name: t_ds_audit_log; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_audit_log (
    id integer NOT NULL,
    user_id integer NOT NULL,
    resource_type integer NOT NULL,
    operation integer NOT NULL,
    "time" timestamp without time zone,
    resource_id integer NOT NULL
);

ALTER TABLE
    public.t_ds_audit_log OWNER TO root;

--
-- Name: t_ds_audit_log_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_audit_log_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_audit_log_id_seq OWNER TO root;

--
-- Name: t_ds_audit_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_audit_log_id_seq OWNED BY public.t_ds_audit_log.id;

--
-- Name: t_ds_cluster; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_cluster (
    id integer NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    config text,
    description text,
    operator integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_cluster OWNER TO root;

--
-- Name: t_ds_cluster_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_cluster_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_cluster_id_seq OWNER TO root;

--
-- Name: t_ds_cluster_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_cluster_id_seq OWNED BY public.t_ds_cluster.id;

--
-- Name: t_ds_command_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_command_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_command_id_sequence OWNER TO root;

--
-- Name: t_ds_command; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_command (
    id integer DEFAULT nextval('public.t_ds_command_id_sequence' :: regclass) NOT NULL,
    command_type integer,
    process_definition_code bigint NOT NULL,
    command_param text,
    task_depend_type integer,
    failure_strategy integer DEFAULT 0,
    warning_type integer DEFAULT 0,
    warning_group_id integer,
    schedule_time timestamp without time zone,
    start_time timestamp without time zone,
    executor_id integer,
    update_time timestamp without time zone,
    process_instance_priority integer DEFAULT 2,
    worker_group character varying(255),
    tenant_code character varying(64) DEFAULT 'default' :: character varying,
    environment_code bigint DEFAULT '-1' :: bigint,
    dry_run integer DEFAULT 0,
    process_instance_id integer DEFAULT 0,
    process_definition_version integer DEFAULT 0,
    test_flag integer
);

ALTER TABLE
    public.t_ds_command OWNER TO root;

--
-- Name: t_ds_datasource_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_datasource_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_datasource_id_sequence OWNER TO root;

--
-- Name: t_ds_datasource; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_datasource (
    id integer DEFAULT nextval('public.t_ds_datasource_id_sequence' :: regclass) NOT NULL,
    name character varying(64) NOT NULL,
    note character varying(255) DEFAULT NULL :: character varying,
    type integer NOT NULL,
    user_id integer NOT NULL,
    connection_params text NOT NULL,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_datasource OWNER TO root;

--
-- Name: t_ds_dq_comparison_type; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_dq_comparison_type (
    id integer NOT NULL,
    type character varying NOT NULL,
    execute_sql character varying,
    output_table character varying,
    name character varying,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    is_inner_source boolean
);

ALTER TABLE
    public.t_ds_dq_comparison_type OWNER TO root;

--
-- Name: t_ds_dq_comparison_type_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_dq_comparison_type_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_dq_comparison_type_id_seq OWNER TO root;

--
-- Name: t_ds_dq_comparison_type_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_dq_comparison_type_id_seq OWNED BY public.t_ds_dq_comparison_type.id;

--
-- Name: t_ds_dq_execute_result; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_dq_execute_result (
    id integer NOT NULL,
    process_definition_id integer,
    process_instance_id integer,
    task_instance_id integer,
    rule_type integer,
    rule_name character varying(255) DEFAULT NULL :: character varying,
    statistics_value double precision,
    comparison_value double precision,
    check_type integer,
    threshold double precision,
    operator integer,
    failure_strategy integer,
    state integer,
    user_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    comparison_type integer,
    error_output_path text
);

ALTER TABLE
    public.t_ds_dq_execute_result OWNER TO root;

--
-- Name: t_ds_dq_execute_result_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_dq_execute_result_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_dq_execute_result_id_seq OWNER TO root;

--
-- Name: t_ds_dq_execute_result_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_dq_execute_result_id_seq OWNED BY public.t_ds_dq_execute_result.id;

--
-- Name: t_ds_dq_rule; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_dq_rule (
    id integer NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    type integer,
    user_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_dq_rule OWNER TO root;

--
-- Name: t_ds_dq_rule_execute_sql; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_dq_rule_execute_sql (
    id integer NOT NULL,
    index integer,
    sql text,
    table_alias character varying(255) DEFAULT NULL :: character varying,
    type integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    is_error_output_sql boolean
);

ALTER TABLE
    public.t_ds_dq_rule_execute_sql OWNER TO root;

--
-- Name: t_ds_dq_rule_execute_sql_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_dq_rule_execute_sql_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_dq_rule_execute_sql_id_seq OWNER TO root;

--
-- Name: t_ds_dq_rule_execute_sql_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_dq_rule_execute_sql_id_seq OWNED BY public.t_ds_dq_rule_execute_sql.id;

--
-- Name: t_ds_dq_rule_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_dq_rule_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_dq_rule_id_seq OWNER TO root;

--
-- Name: t_ds_dq_rule_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_dq_rule_id_seq OWNED BY public.t_ds_dq_rule.id;

--
-- Name: t_ds_dq_rule_input_entry; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_dq_rule_input_entry (
    id integer NOT NULL,
    field character varying(255) DEFAULT NULL :: character varying,
    type character varying(255) DEFAULT NULL :: character varying,
    title character varying(255) DEFAULT NULL :: character varying,
    value character varying(255) DEFAULT NULL :: character varying,
    options text,
    placeholder character varying(255) DEFAULT NULL :: character varying,
    option_source_type integer,
    value_type integer,
    input_type integer,
    is_show smallint DEFAULT '1' :: smallint,
    can_edit smallint DEFAULT '1' :: smallint,
    is_emit smallint DEFAULT '0' :: smallint,
    is_validate smallint DEFAULT '0' :: smallint,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_dq_rule_input_entry OWNER TO root;

--
-- Name: t_ds_dq_rule_input_entry_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_dq_rule_input_entry_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_dq_rule_input_entry_id_seq OWNER TO root;

--
-- Name: t_ds_dq_rule_input_entry_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_dq_rule_input_entry_id_seq OWNED BY public.t_ds_dq_rule_input_entry.id;

--
-- Name: t_ds_dq_task_statistics_value; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_dq_task_statistics_value (
    id integer NOT NULL,
    process_definition_id integer NOT NULL,
    task_instance_id integer,
    rule_id integer NOT NULL,
    unique_code character varying NOT NULL,
    statistics_name character varying,
    statistics_value double precision,
    data_time timestamp(0) without time zone,
    create_time timestamp(0) without time zone,
    update_time timestamp(0) without time zone
);

ALTER TABLE
    public.t_ds_dq_task_statistics_value OWNER TO root;

--
-- Name: t_ds_dq_task_statistics_value_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_dq_task_statistics_value_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_dq_task_statistics_value_id_seq OWNER TO root;

--
-- Name: t_ds_dq_task_statistics_value_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_dq_task_statistics_value_id_seq OWNED BY public.t_ds_dq_task_statistics_value.id;

--
-- Name: t_ds_environment; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_environment (
    id integer NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    config text,
    description text,
    operator integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_environment OWNER TO root;

--
-- Name: t_ds_environment_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_environment_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_environment_id_seq OWNER TO root;

--
-- Name: t_ds_environment_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_environment_id_seq OWNED BY public.t_ds_environment.id;

--
-- Name: t_ds_environment_worker_group_relation; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_environment_worker_group_relation (
    id integer NOT NULL,
    environment_code bigint NOT NULL,
    worker_group character varying(255) NOT NULL,
    operator integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_environment_worker_group_relation OWNER TO root;

--
-- Name: t_ds_environment_worker_group_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_environment_worker_group_relation_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_environment_worker_group_relation_id_seq OWNER TO root;

--
-- Name: t_ds_environment_worker_group_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_environment_worker_group_relation_id_seq OWNED BY public.t_ds_environment_worker_group_relation.id;

--
-- Name: t_ds_error_command; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_error_command (
    id integer NOT NULL,
    command_type integer,
    process_definition_code bigint NOT NULL,
    command_param text,
    task_depend_type integer,
    failure_strategy integer DEFAULT 0,
    warning_type integer DEFAULT 0,
    warning_group_id integer,
    schedule_time timestamp without time zone,
    start_time timestamp without time zone,
    executor_id integer,
    update_time timestamp without time zone,
    process_instance_priority integer DEFAULT 2,
    worker_group character varying(255),
    tenant_code character varying(64) DEFAULT 'default' :: character varying,
    environment_code bigint DEFAULT '-1' :: bigint,
    dry_run integer DEFAULT 0,
    message text,
    process_instance_id integer DEFAULT 0,
    process_definition_version integer DEFAULT 0,
    test_flag integer
);

ALTER TABLE
    public.t_ds_error_command OWNER TO root;

--
-- Name: t_ds_fav_task; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_fav_task (
    id integer NOT NULL,
    task_type character varying(64) NOT NULL,
    user_id integer NOT NULL
);

ALTER TABLE
    public.t_ds_fav_task OWNER TO root;

--
-- Name: t_ds_fav_task_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_fav_task_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_fav_task_id_seq OWNER TO root;

--
-- Name: t_ds_fav_task_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_fav_task_id_seq OWNED BY public.t_ds_fav_task.id;

--
-- Name: t_ds_k8s; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_k8s (
    id integer NOT NULL,
    k8s_name character varying(255) DEFAULT NULL :: character varying,
    k8s_config text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_k8s OWNER TO root;

--
-- Name: t_ds_k8s_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_k8s_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_k8s_id_seq OWNER TO root;

--
-- Name: t_ds_k8s_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_k8s_id_seq OWNED BY public.t_ds_k8s.id;

--
-- Name: t_ds_k8s_namespace; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_k8s_namespace (
    id integer NOT NULL,
    code bigint NOT NULL,
    limits_memory integer,
    namespace character varying(255) DEFAULT NULL :: character varying,
    user_id integer,
    pod_replicas integer,
    pod_request_cpu numeric(13, 4),
    pod_request_memory integer,
    limits_cpu numeric(13, 4),
    cluster_code bigint NOT NULL,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_k8s_namespace OWNER TO root;

--
-- Name: t_ds_k8s_namespace_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_k8s_namespace_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_k8s_namespace_id_seq OWNER TO root;

--
-- Name: t_ds_k8s_namespace_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_k8s_namespace_id_seq OWNED BY public.t_ds_k8s_namespace.id;

--
-- Name: t_ds_plugin_define; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_plugin_define (
    id integer NOT NULL,
    plugin_name character varying(255) NOT NULL,
    plugin_type character varying(63) NOT NULL,
    plugin_params text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_plugin_define OWNER TO root;

--
-- Name: t_ds_plugin_define_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_plugin_define_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_plugin_define_id_seq OWNER TO root;

--
-- Name: t_ds_plugin_define_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_plugin_define_id_seq OWNED BY public.t_ds_plugin_define.id;

--
-- Name: t_ds_process_definition_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_process_definition_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_process_definition_id_sequence OWNER TO root;

--
-- Name: t_ds_process_definition; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_process_definition (
    id integer DEFAULT nextval(
        'public.t_ds_process_definition_id_sequence' :: regclass
    ) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    version integer NOT NULL,
    description text,
    project_code bigint,
    release_state integer,
    user_id integer,
    global_params text,
    locations text,
    warning_group_id integer,
    flag integer,
    timeout integer DEFAULT 0,
    execution_type integer DEFAULT 0,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_process_definition OWNER TO root;

--
-- Name: t_ds_process_definition_log_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_process_definition_log_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_process_definition_log_id_sequence OWNER TO root;

--
-- Name: t_ds_process_definition_log; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_process_definition_log (
    id integer DEFAULT nextval(
        'public.t_ds_process_definition_log_id_sequence' :: regclass
    ) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    version integer NOT NULL,
    description text,
    project_code bigint,
    release_state integer,
    user_id integer,
    global_params text,
    locations text,
    warning_group_id integer,
    flag integer,
    timeout integer DEFAULT 0,
    execution_type integer DEFAULT 0,
    operator integer,
    operate_time timestamp without time zone,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_process_definition_log OWNER TO root;

--
-- Name: t_ds_process_instance_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_process_instance_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_process_instance_id_sequence OWNER TO root;

--
-- Name: t_ds_process_instance; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_process_instance (
    id integer DEFAULT nextval(
        'public.t_ds_process_instance_id_sequence' :: regclass
    ) NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    process_definition_code bigint,
    process_definition_version integer,
    project_code bigint,
    state integer,
    state_history text,
    recovery integer,
    start_time timestamp without time zone,
    end_time timestamp without time zone,
    run_times integer,
    host character varying(135) DEFAULT NULL :: character varying,
    command_type integer,
    command_param text,
    task_depend_type integer,
    max_try_times integer DEFAULT 0,
    failure_strategy integer DEFAULT 0,
    warning_type integer DEFAULT 0,
    warning_group_id integer,
    schedule_time timestamp without time zone,
    command_start_time timestamp without time zone,
    global_params text,
    process_instance_json text,
    flag integer DEFAULT 1,
    update_time timestamp without time zone,
    is_sub_process integer DEFAULT 0,
    executor_id integer NOT NULL,
    executor_name character varying(64) DEFAULT NULL :: character varying,
    history_cmd text,
    dependence_schedule_times text,
    process_instance_priority integer DEFAULT 2,
    worker_group character varying(255),
    environment_code bigint DEFAULT '-1' :: bigint,
    timeout integer DEFAULT 0,
    tenant_code character varying(64) DEFAULT 'default' :: character varying,
    var_pool text,
    dry_run integer DEFAULT 0,
    next_process_instance_id integer DEFAULT 0,
    restart_time timestamp without time zone,
    test_flag integer
);

ALTER TABLE
    public.t_ds_process_instance OWNER TO root;

--
-- Name: t_ds_process_task_relation_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_process_task_relation_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_process_task_relation_id_sequence OWNER TO root;

--
-- Name: t_ds_process_task_relation; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_process_task_relation (
    id integer DEFAULT nextval(
        'public.t_ds_process_task_relation_id_sequence' :: regclass
    ) NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    project_code bigint,
    process_definition_code bigint,
    process_definition_version integer,
    pre_task_code bigint,
    pre_task_version integer DEFAULT 0,
    post_task_code bigint,
    post_task_version integer DEFAULT 0,
    condition_type integer,
    condition_params text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_process_task_relation OWNER TO root;

--
-- Name: t_ds_process_task_relation_log_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_process_task_relation_log_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_process_task_relation_log_id_sequence OWNER TO root;

--
-- Name: t_ds_process_task_relation_log; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_process_task_relation_log (
    id integer DEFAULT nextval(
        'public.t_ds_process_task_relation_log_id_sequence' :: regclass
    ) NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    project_code bigint,
    process_definition_code bigint,
    process_definition_version integer,
    pre_task_code bigint,
    pre_task_version integer DEFAULT 0,
    post_task_code bigint,
    post_task_version integer DEFAULT 0,
    condition_type integer,
    condition_params text,
    operator integer,
    operate_time timestamp without time zone,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_process_task_relation_log OWNER TO root;

--
-- Name: t_ds_project_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_project_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_project_id_sequence OWNER TO root;

--
-- Name: t_ds_project; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_project (
    id integer DEFAULT nextval('public.t_ds_project_id_sequence' :: regclass) NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    code bigint NOT NULL,
    description character varying(255) DEFAULT NULL :: character varying,
    user_id integer,
    flag integer DEFAULT 1,
    create_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    update_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE
    public.t_ds_project OWNER TO root;

--
-- Name: t_ds_project_parameter_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_project_parameter_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_project_parameter_id_sequence OWNER TO root;

--
-- Name: t_ds_project_parameter; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_project_parameter (
    id integer DEFAULT nextval(
        'public.t_ds_project_parameter_id_sequence' :: regclass
    ) NOT NULL,
    param_name character varying(255) NOT NULL,
    param_value character varying(255) NOT NULL,
    code bigint NOT NULL,
    project_code bigint NOT NULL,
    user_id integer,
    create_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    update_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE
    public.t_ds_project_parameter OWNER TO root;

--
-- Name: t_ds_project_preference_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_project_preference_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_project_preference_id_sequence OWNER TO root;

--
-- Name: t_ds_project_preference; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_project_preference (
    id integer DEFAULT nextval(
        'public.t_ds_project_preference_id_sequence' :: regclass
    ) NOT NULL,
    code bigint NOT NULL,
    project_code bigint NOT NULL,
    preferences character varying(512) NOT NULL,
    user_id integer,
    state integer DEFAULT 1,
    create_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    update_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE
    public.t_ds_project_preference OWNER TO root;

--
-- Name: t_ds_queue_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_queue_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_queue_id_sequence OWNER TO root;

--
-- Name: t_ds_queue; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_queue (
    id integer DEFAULT nextval('public.t_ds_queue_id_sequence' :: regclass) NOT NULL,
    queue_name character varying(64) DEFAULT NULL :: character varying,
    queue character varying(64) DEFAULT NULL :: character varying,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_queue OWNER TO root;

--
-- Name: t_ds_relation_datasource_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_datasource_user_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_datasource_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_datasource_user; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_datasource_user (
    id integer DEFAULT nextval(
        'public.t_ds_relation_datasource_user_id_sequence' :: regclass
    ) NOT NULL,
    user_id integer NOT NULL,
    datasource_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_datasource_user OWNER TO root;

--
-- Name: t_ds_relation_namespace_user; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_namespace_user (
    id integer NOT NULL,
    user_id integer,
    namespace_id integer,
    perm integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_namespace_user OWNER TO root;

--
-- Name: t_ds_relation_namespace_user_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_namespace_user_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_namespace_user_id_seq OWNER TO root;

--
-- Name: t_ds_relation_namespace_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_relation_namespace_user_id_seq OWNED BY public.t_ds_relation_namespace_user.id;

--
-- Name: t_ds_relation_process_instance_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_process_instance_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_process_instance_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_process_instance; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_process_instance (
    id integer DEFAULT nextval(
        'public.t_ds_relation_process_instance_id_sequence' :: regclass
    ) NOT NULL,
    parent_process_instance_id integer,
    parent_task_instance_id integer,
    process_instance_id integer
);

ALTER TABLE
    public.t_ds_relation_process_instance OWNER TO root;

--
-- Name: t_ds_relation_project_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_project_user_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_project_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_project_user; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_project_user (
    id integer DEFAULT nextval(
        'public.t_ds_relation_project_user_id_sequence' :: regclass
    ) NOT NULL,
    user_id integer NOT NULL,
    project_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_project_user OWNER TO root;

--
-- Name: t_ds_relation_resources_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_resources_user_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_resources_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_resources_user; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_resources_user (
    id integer DEFAULT nextval(
        'public.t_ds_relation_resources_user_id_sequence' :: regclass
    ) NOT NULL,
    user_id integer NOT NULL,
    resources_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_resources_user OWNER TO root;

--
-- Name: t_ds_relation_rule_execute_sql; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_rule_execute_sql (
    id integer NOT NULL,
    rule_id integer,
    execute_sql_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_rule_execute_sql OWNER TO root;

--
-- Name: t_ds_relation_rule_execute_sql_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_rule_execute_sql_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_rule_execute_sql_id_seq OWNER TO root;

--
-- Name: t_ds_relation_rule_execute_sql_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_relation_rule_execute_sql_id_seq OWNED BY public.t_ds_relation_rule_execute_sql.id;

--
-- Name: t_ds_relation_rule_input_entry; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_rule_input_entry (
    id integer NOT NULL,
    rule_id integer,
    rule_input_entry_id integer,
    values_map text,
    index integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_rule_input_entry OWNER TO root;

--
-- Name: t_ds_relation_rule_input_entry_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_rule_input_entry_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_rule_input_entry_id_seq OWNER TO root;

--
-- Name: t_ds_relation_rule_input_entry_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_relation_rule_input_entry_id_seq OWNED BY public.t_ds_relation_rule_input_entry.id;

--
-- Name: t_ds_relation_sub_workflow; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_sub_workflow (
    id integer NOT NULL,
    parent_workflow_instance_id bigint NOT NULL,
    parent_task_code bigint NOT NULL,
    sub_workflow_instance_id bigint NOT NULL
);

ALTER TABLE
    public.t_ds_relation_sub_workflow OWNER TO root;

--
-- Name: t_ds_relation_sub_workflow_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_sub_workflow_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_sub_workflow_id_seq OWNER TO root;

--
-- Name: t_ds_relation_sub_workflow_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_relation_sub_workflow_id_seq OWNED BY public.t_ds_relation_sub_workflow.id;

--
-- Name: t_ds_relation_udfs_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_relation_udfs_user_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_relation_udfs_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_udfs_user; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_relation_udfs_user (
    id integer DEFAULT nextval(
        'public.t_ds_relation_udfs_user_id_sequence' :: regclass
    ) NOT NULL,
    user_id integer NOT NULL,
    udf_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_relation_udfs_user OWNER TO root;

--
-- Name: t_ds_resources_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_resources_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_resources_id_sequence OWNER TO root;

--
-- Name: t_ds_resources; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_resources (
    id integer DEFAULT nextval('public.t_ds_resources_id_sequence' :: regclass) NOT NULL,
    alias character varying(64) DEFAULT NULL :: character varying,
    file_name character varying(64) DEFAULT NULL :: character varying,
    description character varying(255) DEFAULT NULL :: character varying,
    user_id integer,
    type integer,
    size bigint,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    pid integer,
    full_name character varying(128),
    is_directory boolean DEFAULT false
);

ALTER TABLE
    public.t_ds_resources OWNER TO root;

--
-- Name: t_ds_schedules_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_schedules_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_schedules_id_sequence OWNER TO root;

--
-- Name: t_ds_schedules; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_schedules (
    id integer DEFAULT nextval('public.t_ds_schedules_id_sequence' :: regclass) NOT NULL,
    process_definition_code bigint NOT NULL,
    start_time timestamp without time zone NOT NULL,
    end_time timestamp without time zone NOT NULL,
    timezone_id character varying(40) DEFAULT NULL :: character varying,
    crontab character varying(255) NOT NULL,
    failure_strategy integer NOT NULL,
    user_id integer NOT NULL,
    release_state integer NOT NULL,
    warning_type integer NOT NULL,
    warning_group_id integer,
    process_instance_priority integer DEFAULT 2,
    worker_group character varying(255),
    tenant_code character varying(64) DEFAULT 'default' :: character varying,
    environment_code bigint DEFAULT '-1' :: bigint,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone NOT NULL
);

ALTER TABLE
    public.t_ds_schedules OWNER TO root;

--
-- Name: t_ds_session; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_session (
    id character varying(64) NOT NULL,
    user_id integer,
    ip character varying(45) DEFAULT NULL :: character varying,
    last_login_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_session OWNER TO root;

--
-- Name: t_ds_task_definition_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_task_definition_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_task_definition_id_sequence OWNER TO root;

--
-- Name: t_ds_task_definition; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_task_definition (
    id integer DEFAULT nextval(
        'public.t_ds_task_definition_id_sequence' :: regclass
    ) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    version integer NOT NULL,
    description text,
    project_code bigint,
    user_id integer,
    task_type character varying(50) DEFAULT NULL :: character varying,
    task_execute_type integer DEFAULT 0,
    task_params text,
    flag integer,
    is_cache integer DEFAULT 0,
    task_priority integer DEFAULT 2,
    worker_group character varying(255) DEFAULT NULL :: character varying,
    environment_code bigint DEFAULT '-1' :: bigint,
    fail_retry_times integer,
    fail_retry_interval integer,
    timeout_flag integer,
    timeout_notify_strategy integer,
    timeout integer DEFAULT 0,
    delay_time integer DEFAULT 0,
    task_group_id integer,
    task_group_priority integer DEFAULT 0,
    resource_ids text,
    cpu_quota integer DEFAULT '-1' :: integer NOT NULL,
    memory_max integer DEFAULT '-1' :: integer NOT NULL,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_task_definition OWNER TO root;

--
-- Name: t_ds_task_definition_log_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_task_definition_log_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_task_definition_log_id_sequence OWNER TO root;

--
-- Name: t_ds_task_definition_log; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_task_definition_log (
    id integer DEFAULT nextval(
        'public.t_ds_task_definition_log_id_sequence' :: regclass
    ) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    version integer NOT NULL,
    description text,
    project_code bigint,
    user_id integer,
    task_type character varying(50) DEFAULT NULL :: character varying,
    task_execute_type integer DEFAULT 0,
    task_params text,
    flag integer,
    is_cache integer DEFAULT 0,
    task_priority integer DEFAULT 2,
    worker_group character varying(255) DEFAULT NULL :: character varying,
    environment_code bigint DEFAULT '-1' :: bigint,
    fail_retry_times integer,
    fail_retry_interval integer,
    timeout_flag integer,
    timeout_notify_strategy integer,
    timeout integer DEFAULT 0,
    delay_time integer DEFAULT 0,
    resource_ids text,
    operator integer,
    task_group_id integer,
    task_group_priority integer DEFAULT 0,
    operate_time timestamp without time zone,
    cpu_quota integer DEFAULT '-1' :: integer NOT NULL,
    memory_max integer DEFAULT '-1' :: integer NOT NULL,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_task_definition_log OWNER TO root;

--
-- Name: t_ds_task_group; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_task_group (
    id integer NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    description character varying(255) DEFAULT NULL :: character varying,
    group_size integer NOT NULL,
    project_code bigint DEFAULT '0' :: bigint,
    use_size integer DEFAULT 0,
    user_id integer,
    status integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_task_group OWNER TO root;

--
-- Name: t_ds_task_group_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_task_group_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_task_group_id_seq OWNER TO root;

--
-- Name: t_ds_task_group_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_task_group_id_seq OWNED BY public.t_ds_task_group.id;

--
-- Name: t_ds_task_group_queue; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_task_group_queue (
    id integer NOT NULL,
    task_id integer,
    task_name character varying(255) DEFAULT NULL :: character varying,
    group_id integer,
    process_id integer,
    priority integer DEFAULT 0,
    status integer DEFAULT '-1' :: integer,
    force_start integer DEFAULT 0,
    in_queue integer DEFAULT 0,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_task_group_queue OWNER TO root;

--
-- Name: t_ds_task_group_queue_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_task_group_queue_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_task_group_queue_id_seq OWNER TO root;

--
-- Name: t_ds_task_group_queue_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_task_group_queue_id_seq OWNED BY public.t_ds_task_group_queue.id;

--
-- Name: t_ds_task_instance_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_task_instance_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_task_instance_id_sequence OWNER TO root;

--
-- Name: t_ds_task_instance; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_task_instance (
    id integer DEFAULT nextval(
        'public.t_ds_task_instance_id_sequence' :: regclass
    ) NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    task_type character varying(50) DEFAULT NULL :: character varying,
    task_execute_type integer DEFAULT 0,
    task_code bigint NOT NULL,
    task_definition_version integer,
    process_instance_id integer,
    process_instance_name character varying(255) DEFAULT NULL :: character varying,
    project_code bigint,
    state integer,
    submit_time timestamp without time zone,
    start_time timestamp without time zone,
    end_time timestamp without time zone,
    host character varying(135) DEFAULT NULL :: character varying,
    execute_path character varying(200) DEFAULT NULL :: character varying,
    log_path text,
    alert_flag integer,
    retry_times integer DEFAULT 0,
    pid integer,
    app_link text,
    task_params text,
    flag integer DEFAULT 1,
    is_cache integer DEFAULT 0,
    cache_key character varying(200) DEFAULT NULL :: character varying,
    retry_interval integer,
    max_retry_times integer,
    task_instance_priority integer,
    worker_group character varying(255),
    environment_code bigint DEFAULT '-1' :: bigint,
    environment_config text,
    executor_id integer,
    executor_name character varying(64) DEFAULT NULL :: character varying,
    first_submit_time timestamp without time zone,
    delay_time integer DEFAULT 0,
    task_group_id integer,
    var_pool text,
    dry_run integer DEFAULT 0,
    cpu_quota integer DEFAULT '-1' :: integer NOT NULL,
    memory_max integer DEFAULT '-1' :: integer NOT NULL,
    test_flag integer
);

ALTER TABLE
    public.t_ds_task_instance OWNER TO root;

--
-- Name: t_ds_tenant_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_tenant_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_tenant_id_sequence OWNER TO root;

--
-- Name: t_ds_tenant; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_tenant (
    id integer DEFAULT nextval('public.t_ds_tenant_id_sequence' :: regclass) NOT NULL,
    tenant_code character varying(64) DEFAULT NULL :: character varying,
    description character varying(255) DEFAULT NULL :: character varying,
    queue_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_tenant OWNER TO root;

--
-- Name: t_ds_trigger_relation; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_trigger_relation (
    id integer NOT NULL,
    trigger_type integer NOT NULL,
    trigger_code bigint NOT NULL,
    job_id bigint NOT NULL,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_trigger_relation OWNER TO root;

--
-- Name: t_ds_trigger_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_trigger_relation_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_trigger_relation_id_seq OWNER TO root;

--
-- Name: t_ds_trigger_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--
ALTER SEQUENCE public.t_ds_trigger_relation_id_seq OWNED BY public.t_ds_trigger_relation.id;

--
-- Name: t_ds_udfs_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_udfs_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_udfs_id_sequence OWNER TO root;

--
-- Name: t_ds_udfs; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_udfs (
    id integer DEFAULT nextval('public.t_ds_udfs_id_sequence' :: regclass) NOT NULL,
    user_id integer NOT NULL,
    func_name character varying(255) NOT NULL,
    class_name character varying(255) NOT NULL,
    type integer NOT NULL,
    arg_types character varying(255) DEFAULT NULL :: character varying,
    database character varying(255) DEFAULT NULL :: character varying,
    description character varying(255) DEFAULT NULL :: character varying,
    resource_id integer NOT NULL,
    resource_name character varying(255) NOT NULL,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone NOT NULL
);

ALTER TABLE
    public.t_ds_udfs OWNER TO root;

--
-- Name: t_ds_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_user_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_user_id_sequence OWNER TO root;

--
-- Name: t_ds_user; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_user (
    id integer DEFAULT nextval('public.t_ds_user_id_sequence' :: regclass) NOT NULL,
    user_name character varying(64) DEFAULT NULL :: character varying,
    user_password character varying(64) DEFAULT NULL :: character varying,
    user_type integer,
    email character varying(64) DEFAULT NULL :: character varying,
    phone character varying(11) DEFAULT NULL :: character varying,
    tenant_id integer DEFAULT '-1' :: integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    queue character varying(64) DEFAULT NULL :: character varying,
    state integer DEFAULT 1,
    time_zone character varying(32) DEFAULT NULL :: character varying
);

ALTER TABLE
    public.t_ds_user OWNER TO root;

--
-- Name: COLUMN t_ds_user.state; Type: COMMENT; Schema: public; Owner: root
--
COMMENT ON COLUMN public.t_ds_user.state IS 'state 0:disable 1:enable';

--
-- Name: t_ds_version_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_version_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_version_id_sequence OWNER TO root;

--
-- Name: t_ds_version; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_version (
    id integer DEFAULT nextval('public.t_ds_version_id_sequence' :: regclass) NOT NULL,
    version character varying(63) NOT NULL
);

ALTER TABLE
    public.t_ds_version OWNER TO root;

--
-- Name: t_ds_worker_group_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--
CREATE SEQUENCE public.t_ds_worker_group_id_sequence START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;

ALTER TABLE
    public.t_ds_worker_group_id_sequence OWNER TO root;

--
-- Name: t_ds_worker_group; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_worker_group (
    id bigint DEFAULT nextval(
        'public.t_ds_worker_group_id_sequence' :: regclass
    ) NOT NULL,
    name character varying(255) NOT NULL,
    addr_list text,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    description text,
    other_params_json text
);

ALTER TABLE
    public.t_ds_worker_group OWNER TO root;

--
-- Name: t_ds_alert_plugin_instance id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alert_plugin_instance
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_alert_plugin_instance_id_seq' :: regclass
    );

--
-- Name: t_ds_alert_send_status id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alert_send_status
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_alert_send_status_id_seq' :: regclass
    );

--
-- Name: t_ds_audit_log id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_audit_log
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_audit_log_id_seq' :: regclass);

--
-- Name: t_ds_cluster id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_cluster
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_cluster_id_seq' :: regclass);

--
-- Name: t_ds_dq_comparison_type id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_comparison_type
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_dq_comparison_type_id_seq' :: regclass
    );

--
-- Name: t_ds_dq_execute_result id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_execute_result
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_dq_execute_result_id_seq' :: regclass
    );

--
-- Name: t_ds_dq_rule id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_rule
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_dq_rule_id_seq' :: regclass);

--
-- Name: t_ds_dq_rule_execute_sql id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_rule_execute_sql
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_dq_rule_execute_sql_id_seq' :: regclass
    );

--
-- Name: t_ds_dq_rule_input_entry id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_rule_input_entry
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_dq_rule_input_entry_id_seq' :: regclass
    );

--
-- Name: t_ds_dq_task_statistics_value id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_task_statistics_value
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_dq_task_statistics_value_id_seq' :: regclass
    );

--
-- Name: t_ds_environment id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_environment_id_seq' :: regclass);

--
-- Name: t_ds_environment_worker_group_relation id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment_worker_group_relation
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_environment_worker_group_relation_id_seq' :: regclass
    );

--
-- Name: t_ds_fav_task id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_fav_task
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_fav_task_id_seq' :: regclass);

--
-- Name: t_ds_k8s id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_k8s
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_k8s_id_seq' :: regclass);

--
-- Name: t_ds_k8s_namespace id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_k8s_namespace
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_k8s_namespace_id_seq' :: regclass);

--
-- Name: t_ds_plugin_define id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_plugin_define
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_plugin_define_id_seq' :: regclass);

--
-- Name: t_ds_relation_namespace_user id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_namespace_user
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_relation_namespace_user_id_seq' :: regclass
    );

--
-- Name: t_ds_relation_rule_execute_sql id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_rule_execute_sql
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_relation_rule_execute_sql_id_seq' :: regclass
    );

--
-- Name: t_ds_relation_rule_input_entry id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_rule_input_entry
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_relation_rule_input_entry_id_seq' :: regclass
    );

--
-- Name: t_ds_relation_sub_workflow id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_sub_workflow
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_relation_sub_workflow_id_seq' :: regclass
    );

--
-- Name: t_ds_task_group id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_group
ALTER COLUMN
    id
SET
    DEFAULT nextval('public.t_ds_task_group_id_seq' :: regclass);

--
-- Name: t_ds_task_group_queue id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_group_queue
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_task_group_queue_id_seq' :: regclass
    );

--
-- Name: t_ds_trigger_relation id; Type: DEFAULT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_trigger_relation
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_trigger_relation_id_seq' :: regclass
    );

--
-- Data for Name: qrtz_blob_triggers; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_calendars; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_cron_triggers; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_fired_triggers; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_job_details; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_locks; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.qrtz_locks
VALUES
    ('DolphinScheduler', 'STATE_ACCESS');

INSERT INTO
    public.qrtz_locks
VALUES
    ('DolphinScheduler', 'TRIGGER_ACCESS');

--
-- Data for Name: qrtz_paused_trigger_grps; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_scheduler_state; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.qrtz_scheduler_state
VALUES
    (
        'DolphinScheduler',
        '94e4e28622931699347929924',
        1699927102585,
        5000
    );

--
-- Data for Name: qrtz_simple_triggers; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_simprop_triggers; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: qrtz_triggers; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_access_token; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_alert; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_alert_plugin_instance; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_alert_send_status; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_alertgroup; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_alertgroup
VALUES
    (
        1,
        NULL,
        1,
        'default admin warning group',
        'default admin warning group',
        '2018-11-29 10:20:39',
        '2018-11-29 10:20:39'
    );

--
-- Data for Name: t_ds_audit_log; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_cluster; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_command; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_datasource; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_dq_comparison_type; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        1,
        'FixValue',
        NULL,
        NULL,
        NULL,
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        false
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        2,
        'DailyAvg',
        'select round(avg(statistics_value),2) as day_avg from t_ds_dq_task_statistics_value where data_time >=date_trunc(''DAY'', ${data_time}) and data_time < date_add(date_trunc(''day'', ${data_time}),1) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''',
        'day_range',
        'day_range.day_avg',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        true
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        3,
        'WeeklyAvg',
        'select round(avg(statistics_value),2) as week_avg from t_ds_dq_task_statistics_value where  data_time >= date_trunc(''WEEK'', ${data_time}) and data_time <date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''',
        'week_range',
        'week_range.week_avg',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        true
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        4,
        'MonthlyAvg',
        'select round(avg(statistics_value),2) as month_avg from t_ds_dq_task_statistics_value where  data_time >= date_trunc(''MONTH'', ${data_time}) and data_time <date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''',
        'month_range',
        'month_range.month_avg',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        true
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        5,
        'Last7DayAvg',
        'select round(avg(statistics_value),2) as last_7_avg from t_ds_dq_task_statistics_value where  data_time >= date_add(date_trunc(''day'', ${data_time}),-7) and  data_time <date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''',
        'last_seven_days',
        'last_seven_days.last_7_avg',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        true
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        6,
        'Last30DayAvg',
        'select round(avg(statistics_value),2) as last_30_avg from t_ds_dq_task_statistics_value where  data_time >= date_add(date_trunc(''day'', ${data_time}),-30) and  data_time < date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''',
        'last_thirty_days',
        'last_thirty_days.last_30_avg',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        true
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        7,
        'SrcTableTotalRows',
        'SELECT COUNT(*) AS total FROM ${src_table} WHERE (${src_filter})',
        'total_count',
        'total_count.total',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        false
    );

INSERT INTO
    public.t_ds_dq_comparison_type
VALUES
    (
        8,
        'TargetTableTotalRows',
        'SELECT COUNT(*) AS total FROM ${target_table} WHERE (${target_filter})',
        'total_count',
        'total_count.total',
        '2021-06-30 00:00:00',
        '2021-06-30 00:00:00',
        false
    );

--
-- Data for Name: t_ds_dq_execute_result; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_dq_rule; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        1,
        '$t(null_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        2,
        '$t(custom_sql)',
        1,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        3,
        '$t(multi_table_accuracy)',
        2,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        4,
        '$t(multi_table_value_comparison)',
        3,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        5,
        '$t(field_length_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        6,
        '$t(uniqueness_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        7,
        '$t(regexp_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        8,
        '$t(timeliness_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        9,
        '$t(enumeration_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

INSERT INTO
    public.t_ds_dq_rule
VALUES
    (
        10,
        '$t(table_count_check)',
        0,
        1,
        '2020-01-12 00:00:00',
        '2020-01-12 00:00:00'
    );

--
-- Data for Name: t_ds_dq_rule_execute_sql; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        1,
        1,
        'SELECT COUNT(*) AS nulls FROM null_items',
        'null_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        2,
        1,
        'SELECT COUNT(*) AS total FROM ${src_table} WHERE (${src_filter})',
        'total_count',
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        3,
        1,
        'SELECT COUNT(*) AS miss from miss_items',
        'miss_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        4,
        1,
        'SELECT COUNT(*) AS valids FROM invalid_length_items',
        'invalid_length_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        5,
        1,
        'SELECT COUNT(*) AS total FROM ${target_table} WHERE (${target_filter})',
        'total_count',
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        6,
        1,
        'SELECT ${src_field} FROM ${src_table} group by ${src_field} having count(*) > 1',
        'duplicate_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        7,
        1,
        'SELECT COUNT(*) AS duplicates FROM duplicate_items',
        'duplicate_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        8,
        1,
        'SELECT ${src_table}.* FROM (SELECT * FROM ${src_table} WHERE (${src_filter})) ${src_table} LEFT JOIN (SELECT * FROM ${target_table} WHERE (${target_filter})) ${target_table} ON ${on_clause} WHERE ${where_clause}',
        'miss_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        9,
        1,
        'SELECT * FROM ${src_table} WHERE (${src_field} not regexp ''${regexp_pattern}'') AND (${src_filter}) ',
        'regexp_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        10,
        1,
        'SELECT COUNT(*) AS regexps FROM regexp_items',
        'regexp_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        11,
        1,
        'SELECT * FROM ${src_table} WHERE (to_unix_timestamp(${src_field}, ''${datetime_format}'')-to_unix_timestamp(''${deadline}'', ''${datetime_format}'') <= 0) AND (to_unix_timestamp(${src_field}, ''${datetime_format}'')-to_unix_timestamp(''${begin_time}'', ''${datetime_format}'') >= 0) AND (${src_filter}) ',
        'timeliness_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        12,
        1,
        'SELECT COUNT(*) AS timeliness FROM timeliness_items',
        'timeliness_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        13,
        1,
        'SELECT * FROM ${src_table} where (${src_field} not in ( ${enum_list} ) or ${src_field} is null) AND (${src_filter}) ',
        'enum_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        14,
        1,
        'SELECT COUNT(*) AS enums FROM enum_items',
        'enum_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        15,
        1,
        'SELECT COUNT(*) AS total FROM ${src_table} WHERE (${src_filter})',
        'table_count',
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        false
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        16,
        1,
        'SELECT * FROM ${src_table} WHERE (${src_field} is null or ${src_field} = '''') AND (${src_filter})',
        'null_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

INSERT INTO
    public.t_ds_dq_rule_execute_sql
VALUES
    (
        17,
        1,
        'SELECT * FROM ${src_table} WHERE (length(${src_field}) ${logic_operator} ${field_length}) AND (${src_filter})',
        'invalid_length_items',
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24',
        true
    );

--
-- Data for Name: t_ds_dq_rule_input_entry; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        1,
        'src_connector_type',
        'select',
        '$t(src_connector_type)',
        '',
        '[{"label":"HIVE","value":"HIVE"},{"label":"JDBC","value":"JDBC"}]',
        'please select source connector type',
        2,
        2,
        0,
        1,
        1,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        2,
        'src_datasource_id',
        'select',
        '$t(src_datasource_id)',
        '',
        NULL,
        'please select source datasource id',
        1,
        2,
        0,
        1,
        1,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        3,
        'src_table',
        'select',
        '$t(src_table)',
        NULL,
        NULL,
        'Please enter source table name',
        0,
        0,
        0,
        1,
        1,
        1,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        4,
        'src_filter',
        'input',
        '$t(src_filter)',
        NULL,
        NULL,
        'Please enter filter expression',
        0,
        3,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        5,
        'src_field',
        'select',
        '$t(src_field)',
        NULL,
        NULL,
        'Please enter column, only single column is supported',
        0,
        0,
        0,
        1,
        1,
        0,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        6,
        'statistics_name',
        'input',
        '$t(statistics_name)',
        NULL,
        NULL,
        'Please enter statistics name, the alias in statistics execute sql',
        0,
        0,
        1,
        0,
        0,
        0,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        7,
        'check_type',
        'select',
        '$t(check_type)',
        '0',
        '[{"label":"Expected - Actual","value":"0"},{"label":"Actual - Expected","value":"1"},{"label":"Actual / Expected","value":"2"},{"label":"(Expected - Actual) / Expected","value":"3"}]',
        'please select check type',
        0,
        0,
        3,
        1,
        1,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        8,
        'operator',
        'select',
        '$t(operator)',
        '0',
        '[{"label":"=","value":"0"},{"label":"<","value":"1"},{"label":"<=","value":"2"},{"label":">","value":"3"},{"label":">=","value":"4"},{"label":"!=","value":"5"}]',
        'please select operator',
        0,
        0,
        3,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        9,
        'threshold',
        'input',
        '$t(threshold)',
        NULL,
        NULL,
        'Please enter threshold, number is needed',
        0,
        2,
        3,
        1,
        1,
        0,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        10,
        'failure_strategy',
        'select',
        '$t(failure_strategy)',
        '0',
        '[{"label":"Alert","value":"0"},{"label":"Block","value":"1"}]',
        'please select failure strategy',
        0,
        0,
        3,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        11,
        'target_connector_type',
        'select',
        '$t(target_connector_type)',
        '',
        '[{"label":"HIVE","value":"HIVE"},{"label":"JDBC","value":"JDBC"}]',
        'Please select target connector type',
        2,
        0,
        0,
        1,
        1,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        12,
        'target_datasource_id',
        'select',
        '$t(target_datasource_id)',
        '',
        NULL,
        'Please select target datasource',
        1,
        2,
        0,
        1,
        1,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        13,
        'target_table',
        'select',
        '$t(target_table)',
        NULL,
        NULL,
        'Please enter target table',
        0,
        0,
        0,
        1,
        1,
        1,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        14,
        'target_filter',
        'input',
        '$t(target_filter)',
        NULL,
        NULL,
        'Please enter target filter expression',
        0,
        3,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        15,
        'mapping_columns',
        'group',
        '$t(mapping_columns)',
        NULL,
        '[{"field":"src_field","props":{"placeholder":"Please input src field","rows":0,"disabled":false,"size":"small"},"type":"input","title":"src_field"},{"field":"operator","props":{"placeholder":"Please input operator","rows":0,"disabled":false,"size":"small"},"type":"input","title":"operator"},{"field":"target_field","props":{"placeholder":"Please input target field","rows":0,"disabled":false,"size":"small"},"type":"input","title":"target_field"}]',
        'please enter mapping columns',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        16,
        'statistics_execute_sql',
        'textarea',
        '$t(statistics_execute_sql)',
        NULL,
        NULL,
        'Please enter statistics execute sql',
        0,
        3,
        0,
        1,
        1,
        0,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        17,
        'comparison_name',
        'input',
        '$t(comparison_name)',
        NULL,
        NULL,
        'Please enter comparison name, the alias in comparison execute sql',
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        18,
        'comparison_execute_sql',
        'textarea',
        '$t(comparison_execute_sql)',
        NULL,
        NULL,
        'Please enter comparison execute sql',
        0,
        3,
        0,
        1,
        1,
        0,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        19,
        'comparison_type',
        'select',
        '$t(comparison_type)',
        '',
        NULL,
        'Please enter comparison title',
        3,
        0,
        2,
        1,
        0,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        20,
        'writer_connector_type',
        'select',
        '$t(writer_connector_type)',
        '',
        '[{"label":"MYSQL","value":"0"},{"label":"POSTGRESQL","value":"1"}]',
        'please select writer connector type',
        0,
        2,
        0,
        1,
        1,
        1,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        21,
        'writer_datasource_id',
        'select',
        '$t(writer_datasource_id)',
        '',
        NULL,
        'please select writer datasource id',
        1,
        2,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        22,
        'target_field',
        'select',
        '$t(target_field)',
        NULL,
        NULL,
        'Please enter column, only single column is supported',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        23,
        'field_length',
        'input',
        '$t(field_length)',
        NULL,
        NULL,
        'Please enter length limit',
        0,
        3,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        24,
        'logic_operator',
        'select',
        '$t(logic_operator)',
        '=',
        '[{"label":"=","value":"="},{"label":"<","value":"<"},{"label":"<=","value":"<="},{"label":">","value":">"},{"label":">=","value":">="},{"label":"<>","value":"<>"}]',
        'please select logic operator',
        0,
        0,
        3,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        25,
        'regexp_pattern',
        'input',
        '$t(regexp_pattern)',
        NULL,
        NULL,
        'Please enter regexp pattern',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        26,
        'deadline',
        'input',
        '$t(deadline)',
        NULL,
        NULL,
        'Please enter deadline',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        27,
        'datetime_format',
        'input',
        '$t(datetime_format)',
        NULL,
        NULL,
        'Please enter datetime format',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        28,
        'enum_list',
        'input',
        '$t(enum_list)',
        NULL,
        NULL,
        'Please enter enumeration',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        29,
        'begin_time',
        'input',
        '$t(begin_time)',
        NULL,
        NULL,
        'Please enter begin time',
        0,
        0,
        0,
        1,
        1,
        0,
        0,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        30,
        'src_database',
        'select',
        '$t(src_database)',
        NULL,
        NULL,
        'Please select source database',
        0,
        0,
        0,
        1,
        1,
        1,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_dq_rule_input_entry
VALUES
    (
        31,
        'target_database',
        'select',
        '$t(target_database)',
        NULL,
        NULL,
        'Please select target database',
        0,
        0,
        0,
        1,
        1,
        1,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

--
-- Data for Name: t_ds_dq_task_statistics_value; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_environment; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_environment_worker_group_relation; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_error_command; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_fav_task; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_k8s; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_k8s_namespace; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_plugin_define; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        1,
        'Script',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"the custom parameters passed when calling scripts","size":"small"},"field":"userParams","name":"$t(''userParams'')","type":"input","title":"$t(''userParams'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"the absolute script path under alert-server, and make sure access rights","size":"small"},"field":"path","name":"$t(''scriptPath'')","type":"input","title":"$t(''scriptPath'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"type","name":"$t(''scriptType'')","type":"radio","title":"$t(''scriptType'')","value":"SHELL","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"SHELL","value":"SHELL","disabled":false}]}]',
        '2023-11-07 17:04:34.922',
        '2023-11-07 17:04:34.922'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        2,
        'WeChat',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input corp id","size":"small"},"field":"corpId","name":"$t(''corpId'')","type":"input","title":"$t(''corpId'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input secret","size":"small"},"field":"secret","name":"$t(''secret'')","type":"input","title":"$t(''secret'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"use `|` to separate userIds and `@all` to everyone","size":"small"},"field":"users","name":"$t(''users'')","type":"input","title":"$t(''users'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input agent id or chat id","size":"small"},"field":"agentId/chatId","name":"$t(''agentId/chatId'')","type":"input","title":"$t(''agentId/chatId'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"sendType","name":"send.type","type":"radio","title":"send.type","value":"APP/应用","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"APP/应用","value":"APP/应用","disabled":false},{"label":"GROUP CHAT/群聊","value":"GROUP CHAT/群聊","disabled":false}]},{"props":null,"field":"showType","name":"$t(''showType'')","type":"radio","title":"$t(''showType'')","value":"markdown","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"markdown","value":"markdown","disabled":false},{"label":"text","value":"text","disabled":false}]}]',
        '2023-11-07 17:04:35.018',
        '2023-11-07 17:04:35.018'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        3,
        'Telegram',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input WebHook Url","size":"small"},"field":"webHook","name":"$t(''webHook'')","type":"input","title":"$t(''webHook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input bot access token","size":"small"},"field":"botToken","name":"botToken","type":"input","title":"botToken","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input telegram channel chat id","size":"small"},"field":"chatId","name":"chatId","type":"input","title":"chatId","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"field":"parseMode","name":"parseMode","props":{"disabled":null,"placeholder":null,"size":"small"},"type":"select","title":"parseMode","value":"Txt","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"Txt","value":"Txt","disabled":false},{"label":"Markdown","value":"Markdown","disabled":false},{"label":"MarkdownV2","value":"MarkdownV2","disabled":false},{"label":"Html","value":"Html","disabled":false}]},{"props":null,"field":"IsEnableProxy","name":"$t(''isEnableProxy'')","type":"radio","title":"$t(''isEnableProxy'')","value":"false","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"Proxy","name":"$t(''proxy'')","type":"input","title":"$t(''proxy'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Port","name":"$t(''port'')","type":"input-number","title":"$t(''port'')","value":null,"validate":[{"required":false,"message":null,"type":"number","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"User","name":"$t(''user'')","type":"input","title":"$t(''user'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":"password","maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"field":"Password","name":"$t(''password'')","type":"input","title":"$t(''password'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]',
        '2023-11-07 17:04:35.097',
        '2023-11-07 17:04:35.097'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        4,
        'Email',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input receivers","size":"small"},"field":"receivers","name":"$t(''receivers'')","type":"input","title":"$t(''receivers'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"receiverCcs","name":"$t(''receiverCcs'')","type":"input","title":"$t(''receiverCcs'')","value":null,"validate":null,"emit":null},{"props":null,"field":"serverHost","name":"mail.smtp.host","type":"input","title":"mail.smtp.host","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"serverPort","name":"mail.smtp.port","type":"input-number","title":"mail.smtp.port","value":25,"validate":[{"required":true,"message":null,"type":"number","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"sender","name":"$t(''mailSender'')","type":"input","title":"$t(''mailSender'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"enableSmtpAuth","name":"mail.smtp.auth","type":"radio","title":"mail.smtp.auth","value":"true","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"if enable use authentication, you need input user","size":"small"},"field":"User","name":"$t(''mailUser'')","type":"input","title":"$t(''mailUser'')","value":null,"validate":null,"emit":null},{"props":{"disabled":null,"type":"password","maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"field":"Password","name":"$t(''mailPasswd'')","type":"input","title":"$t(''mailPasswd'')","value":null,"validate":null,"emit":null},{"props":null,"field":"starttlsEnable","name":"mail.smtp.starttls.enable","type":"radio","title":"mail.smtp.starttls.enable","value":"false","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"sslEnable","name":"mail.smtp.ssl.enable","type":"radio","title":"mail.smtp.ssl.enable","value":"false","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"smtpSslTrust","name":"mail.smtp.ssl.trust","type":"input","title":"mail.smtp.ssl.trust","value":"*","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"showType","name":"$t(''showType'')","type":"radio","title":"$t(''showType'')","value":"table","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"table","value":"table","disabled":false},{"label":"text","value":"text","disabled":false},{"label":"attachment","value":"attachment","disabled":false},{"label":"table attachment","value":"table attachment","disabled":false}]}]',
        '2023-11-07 17:04:35.108',
        '2023-11-07 17:04:35.108'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        5,
        'Slack',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input WebHook Url","size":"small"},"field":"webHook","name":"$t(''webhook'')","type":"input","title":"$t(''webhook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input the bot username","size":"small"},"field":"username","name":"$t(''Username'')","type":"input","title":"$t(''Username'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]',
        '2023-11-07 17:04:35.13',
        '2023-11-07 17:04:35.13'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        6,
        'Feishu',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":null,"field":"WebHook","name":"$t(''webhook'')","type":"input","title":"$t(''webhook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"IsEnableProxy","name":"$t(''isEnableProxy'')","type":"radio","title":"$t(''isEnableProxy'')","value":"true","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"Proxy","name":"$t(''proxy'')","type":"input","title":"$t(''proxy'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Port","name":"$t(''port'')","type":"input-number","title":"$t(''port'')","value":null,"validate":[{"required":false,"message":null,"type":"number","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"User","name":"$t(''user'')","type":"input","title":"$t(''user'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":"password","maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"field":"Password","name":"$t(''password'')","type":"input","title":"$t(''password'')","value":null,"validate":null,"emit":null}]',
        '2023-11-07 17:04:35.152',
        '2023-11-07 17:04:35.152'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        7,
        'Http',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request URL","size":"small"},"field":"url","name":"$t(''url'')","type":"input","title":"$t(''url'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request type POST or GET","size":"small"},"field":"requestType","name":"$t(''requestType'')","type":"input","title":"$t(''requestType'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request headers as JSON format","size":"small"},"field":"headerParams","name":"$t(''headerParams'')","type":"input","title":"$t(''headerParams'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request body as JSON format","size":"small"},"field":"bodyParams","name":"$t(''bodyParams'')","type":"input","title":"$t(''bodyParams'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input alert msg field name","size":"small"},"field":"contentField","name":"$t(''contentField'')","type":"input","title":"$t(''contentField'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]',
        '2023-11-07 17:04:35.173',
        '2023-11-07 17:04:35.173'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        8,
        'DingTalk',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":null,"field":"WebHook","name":"$t(''webhook'')","type":"input","title":"$t(''webhook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Keyword","name":"$t(''keyword'')","type":"input","title":"$t(''keyword'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Secret","name":"$t(''secret'')","type":"input","title":"$t(''secret'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"MsgType","name":"$t(''msgType'')","type":"radio","title":"$t(''msgType'')","value":"text","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"text","value":"text","disabled":false},{"label":"markdown","value":"markdown","disabled":false}]},{"props":null,"field":"AtMobiles","name":"$t(''atMobiles'')","type":"input","title":"$t(''atMobiles'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"AtUserIds","name":"$t(''atUserIds'')","type":"input","title":"$t(''atUserIds'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"IsAtAll","name":"$t(''isAtAll'')","type":"radio","title":"$t(''isAtAll'')","value":"false","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"IsEnableProxy","name":"$t(''isEnableProxy'')","type":"radio","title":"$t(''isEnableProxy'')","value":"false","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"Proxy","name":"$t(''proxy'')","type":"input","title":"$t(''proxy'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Port","name":"$t(''port'')","type":"input-number","title":"$t(''port'')","value":null,"validate":[{"required":false,"message":null,"type":"number","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"User","name":"$t(''user'')","type":"input","title":"$t(''user'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":"password","maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"field":"Password","name":"$t(''password'')","type":"input","title":"$t(''password'')","value":null,"validate":null,"emit":null}]',
        '2023-11-07 17:04:35.183',
        '2023-11-07 17:04:35.183'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        9,
        'WebexTeams',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input bot access token","size":"small"},"field":"BotAccessToken","name":"botAccessToken","type":"input","title":"botAccessToken","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input the room ID the alert message send to","size":"small"},"field":"RoomId","name":"roomId","type":"input","title":"roomId","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input the person ID of the alert message recipient","size":"small"},"field":"ToPersonId","name":"toPersonId","type":"input","title":"toPersonId","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input the email address of the alert message recipient","size":"small"},"field":"ToPersonEmail","name":"toPersonEmail","type":"input","title":"toPersonEmail","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"use `,`(eng commas) to separate multiple emails, to specify the person you mention in the room","size":"small"},"field":"AtSomeoneInRoom","name":"atSomeoneInRoom","type":"input","title":"atSomeoneInRoom","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Destination","name":"destination","type":"radio","title":"destination","value":"roomId","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"roomId","value":"roomId","disabled":false},{"label":"personEmail","value":"personEmail","disabled":false},{"label":"personId","value":"personId","disabled":false}]}]',
        '2023-11-07 17:04:35.198',
        '2023-11-07 17:04:35.198'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        10,
        'PagerDuty',
        'alert',
        '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":null,"field":"IntegrationKey","name":"integrationKey","type":"input","title":"integrationKey","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]',
        '2023-11-07 17:04:35.207',
        '2023-11-07 17:04:35.207'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        34,
        'JAVA',
        'task',
        'null',
        '2023-11-07 17:05:52.664',
        '2023-11-07 17:05:52.664'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        35,
        'JUPYTER',
        'task',
        'null',
        '2023-11-07 17:05:52.694',
        '2023-11-07 17:05:52.694'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        36,
        'SPARK',
        'task',
        'null',
        '2023-11-07 17:05:52.698',
        '2023-11-07 17:05:52.698'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        37,
        'FLINK_STREAM',
        'task',
        'null',
        '2023-11-07 17:05:52.703',
        '2023-11-07 17:05:52.703'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        38,
        'PYTHON',
        'task',
        'null',
        '2023-11-07 17:05:52.706',
        '2023-11-07 17:05:52.706'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        39,
        'DATASYNC',
        'task',
        '[]',
        '2023-11-07 17:05:52.729',
        '2023-11-07 17:05:52.729'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        40,
        'DATA_FACTORY',
        'task',
        '[]',
        '2023-11-07 17:05:52.739',
        '2023-11-07 17:05:52.739'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        41,
        'CHUNJUN',
        'task',
        'null',
        '2023-11-07 17:05:52.742',
        '2023-11-07 17:05:52.742'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        42,
        'REMOTESHELL',
        'task',
        '[{"props":null,"field":"name","name":"$t(''Node name'')","type":"input","title":"$t(''Node name'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"runFlag","name":"RUN_FLAG","type":"radio","title":"RUN_FLAG","value":null,"validate":null,"emit":null,"options":[{"label":"NORMAL","value":"NORMAL","disabled":false},{"label":"FORBIDDEN","value":"FORBIDDEN","disabled":false}]}]',
        '2023-11-07 17:05:52.845',
        '2023-11-07 17:05:52.845'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        43,
        'PIGEON',
        'task',
        '[{"props":null,"field":"targetJobName","name":"targetJobName","type":"input","title":"targetJobName","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]',
        '2023-11-07 17:05:52.85',
        '2023-11-07 17:05:52.85'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        44,
        'SHELL',
        'task',
        '[{"props":null,"field":"name","name":"$t(''Node name'')","type":"input","title":"$t(''Node name'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"runFlag","name":"RUN_FLAG","type":"radio","title":"RUN_FLAG","value":null,"validate":null,"emit":null,"options":[{"label":"NORMAL","value":"NORMAL","disabled":false},{"label":"FORBIDDEN","value":"FORBIDDEN","disabled":false}]}]',
        '2023-11-07 17:05:52.855',
        '2023-11-07 17:05:52.855'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        45,
        'PROCEDURE',
        'task',
        'null',
        '2023-11-07 17:05:52.858',
        '2023-11-07 17:05:52.858'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        46,
        'SQOOP',
        'task',
        'null',
        '2023-11-07 17:05:52.86',
        '2023-11-07 17:05:52.86'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        47,
        'MR',
        'task',
        'null',
        '2023-11-07 17:05:52.864',
        '2023-11-07 17:05:52.864'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        48,
        'PYTORCH',
        'task',
        '[]',
        '2023-11-07 17:05:52.867',
        '2023-11-07 17:05:52.867'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        49,
        'K8S',
        'task',
        'null',
        '2023-11-07 17:05:52.871',
        '2023-11-07 17:05:52.871'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        50,
        'SAGEMAKER',
        'task',
        '[]',
        '2023-11-07 17:05:52.876',
        '2023-11-07 17:05:52.876'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        51,
        'SEATUNNEL',
        'task',
        'null',
        '2023-11-07 17:05:52.878',
        '2023-11-07 17:05:52.878'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        52,
        'HTTP',
        'task',
        'null',
        '2023-11-07 17:05:52.881',
        '2023-11-07 17:05:52.881'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        53,
        'EMR',
        'task',
        '[]',
        '2023-11-07 17:05:52.883',
        '2023-11-07 17:05:52.883'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        54,
        'DMS',
        'task',
        '[]',
        '2023-11-07 17:05:52.885',
        '2023-11-07 17:05:52.885'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        55,
        'DATA_QUALITY',
        'task',
        'null',
        '2023-11-07 17:05:52.888',
        '2023-11-07 17:05:52.888'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        56,
        'KUBEFLOW',
        'task',
        '[]',
        '2023-11-07 17:05:52.89',
        '2023-11-07 17:05:52.89'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        57,
        'SQL',
        'task',
        'null',
        '2023-11-07 17:05:52.893',
        '2023-11-07 17:05:52.893'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        58,
        'DVC',
        'task',
        '[{"props":null,"field":"name","name":"$t(''Node name'')","type":"input","title":"$t(''Node name'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"runFlag","name":"RUN_FLAG","type":"radio","title":"RUN_FLAG","value":null,"validate":null,"emit":null,"options":[{"label":"NORMAL","value":"NORMAL","disabled":false},{"label":"FORBIDDEN","value":"FORBIDDEN","disabled":false}]}]',
        '2023-11-07 17:05:52.896',
        '2023-11-07 17:05:52.896'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        59,
        'DATAX',
        'task',
        'null',
        '2023-11-07 17:05:52.899',
        '2023-11-07 17:05:52.899'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        60,
        'ZEPPELIN',
        'task',
        'null',
        '2023-11-07 17:05:52.903',
        '2023-11-07 17:05:52.903'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        61,
        'DINKY',
        'task',
        '[]',
        '2023-11-07 17:05:52.913',
        '2023-11-07 17:05:52.913'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        62,
        'MLFLOW',
        'task',
        '[{"props":null,"field":"name","name":"$t(''Node name'')","type":"input","title":"$t(''Node name'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"runFlag","name":"RUN_FLAG","type":"radio","title":"RUN_FLAG","value":null,"validate":null,"emit":null,"options":[{"label":"NORMAL","value":"NORMAL","disabled":false},{"label":"FORBIDDEN","value":"FORBIDDEN","disabled":false}]}]',
        '2023-11-07 17:05:52.918',
        '2023-11-07 17:05:52.918'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        63,
        'OPENMLDB',
        'task',
        'null',
        '2023-11-07 17:05:52.921',
        '2023-11-07 17:05:52.921'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        64,
        'LINKIS',
        'task',
        'null',
        '2023-11-07 17:05:52.923',
        '2023-11-07 17:05:52.923'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        65,
        'FLINK',
        'task',
        'null',
        '2023-11-07 17:05:52.926',
        '2023-11-07 17:05:52.926'
    );

INSERT INTO
    public.t_ds_plugin_define
VALUES
    (
        66,
        'HIVECLI',
        'task',
        'null',
        '2023-11-07 17:05:52.929',
        '2023-11-07 17:05:52.929'
    );

--
-- Data for Name: t_ds_process_definition; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_process_definition_log; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_process_instance; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_process_task_relation; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_process_task_relation_log; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_project; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_project
VALUES
    (
        1,
        'test',
        11572261263264,
        '',
        1,
        1,
        '2023-11-13 09:24:51.112',
        '2023-11-13 09:24:51.112'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        2,
        'test1',
        11572266441760,
        '',
        1,
        1,
        '2023-11-13 09:25:31.576',
        '2023-11-13 09:25:31.576'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        3,
        'ssss',
        11572498688544,
        'sss',
        1,
        1,
        '2023-11-13 09:55:46.005',
        '2023-11-13 09:55:46.005'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        4,
        'sadasd',
        11573872077856,
        'aaaa',
        1,
        1,
        '2023-11-13 12:54:35.608',
        '2023-11-13 12:54:35.608'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        5,
        'ssssasdfasdf',
        11573985504288,
        'adsfasdf',
        1,
        1,
        '2023-11-13 13:09:21.752',
        '2023-11-13 13:09:21.752'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        6,
        'asdf',
        11574585717536,
        '',
        1,
        1,
        '2023-11-13 14:27:30.918',
        '2023-11-13 14:27:30.918'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        7,
        'asdfs',
        11574586707744,
        '',
        1,
        1,
        '2023-11-13 14:27:38.654',
        '2023-11-13 14:27:38.654'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        8,
        '112323',
        11574587347104,
        '',
        1,
        1,
        '2023-11-13 14:27:43.649',
        '2023-11-13 14:27:43.649'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        9,
        '123',
        11574587851040,
        '',
        1,
        1,
        '2023-11-13 14:27:47.586',
        '2023-11-13 14:27:47.586'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        10,
        '333',
        11574588372128,
        '',
        1,
        1,
        '2023-11-13 14:27:51.657',
        '2023-11-13 14:27:51.657'
    );

INSERT INTO
    public.t_ds_project
VALUES
    (
        11,
        '4444',
        11574588974624,
        '',
        1,
        1,
        '2023-11-13 14:27:56.364',
        '2023-11-13 14:27:56.364'
    );

--
-- Data for Name: t_ds_project_parameter; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_project_parameter
VALUES
    (
        1,
        '111',
        '2222',
        11583360942752,
        11574588974624,
        1,
        '2023-11-14 09:30:07.364',
        '2023-11-14 09:30:07.364'
    );

--
-- Data for Name: t_ds_project_preference; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_project_preference
VALUES
    (
        1,
        11583362883616,
        11574588974624,
        '{"taskPriority":"MEDIUM","workerGroup":"default","environmentCode":null,"failRetryTimes":0,"failRetryInterval":1,"cpuQuota":-1,"memoryMax":-1,"timeoutFlag":false,"timeoutNotifyStrategy":["WARN"],"timeout":30,"warningType":"NONE","tenant":"default","alertGroups":1}',
        1,
        1,
        '2023-11-14 09:30:22.527',
        '2023-11-14 09:30:31.177'
    );

--
-- Data for Name: t_ds_queue; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_queue
VALUES
    (
        1,
        'default',
        'default',
        '2018-11-29 10:22:33',
        '2018-11-29 10:22:33'
    );

--
-- Data for Name: t_ds_relation_datasource_user; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_relation_namespace_user; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_relation_process_instance; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_relation_project_user; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_relation_resources_user; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_relation_rule_execute_sql; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        1,
        1,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        3,
        5,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        2,
        3,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        4,
        3,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        5,
        6,
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        6,
        6,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        7,
        7,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        8,
        7,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        9,
        8,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        10,
        8,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        11,
        9,
        13,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        12,
        9,
        14,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        13,
        10,
        15,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        14,
        1,
        16,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_execute_sql
VALUES
    (
        15,
        5,
        17,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

--
-- Data for Name: t_ds_relation_rule_input_entry; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        1,
        1,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        2,
        1,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        3,
        1,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        4,
        1,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        5,
        1,
        5,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        6,
        1,
        6,
        '{"statistics_name":"null_count.nulls"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        7,
        1,
        7,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        8,
        1,
        8,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        9,
        1,
        9,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        10,
        1,
        10,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        11,
        1,
        17,
        '',
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        12,
        1,
        19,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        13,
        2,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        14,
        2,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        15,
        2,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        16,
        2,
        6,
        '{"is_show":"true","can_edit":"true"}',
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        17,
        2,
        16,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        18,
        2,
        4,
        NULL,
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        19,
        2,
        7,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        20,
        2,
        8,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        21,
        2,
        9,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        22,
        2,
        10,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        24,
        2,
        19,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        25,
        3,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        26,
        3,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        27,
        3,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        28,
        3,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        29,
        3,
        11,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        30,
        3,
        12,
        NULL,
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        31,
        3,
        13,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        32,
        3,
        14,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        33,
        3,
        15,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        34,
        3,
        7,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        35,
        3,
        8,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        36,
        3,
        9,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        37,
        3,
        10,
        NULL,
        13,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        38,
        3,
        17,
        '{"comparison_name":"total_count.total"}',
        14,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        39,
        3,
        19,
        NULL,
        15,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        40,
        4,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        41,
        4,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        42,
        4,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        43,
        4,
        6,
        '{"is_show":"true","can_edit":"true"}',
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        44,
        4,
        16,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        45,
        4,
        11,
        NULL,
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        46,
        4,
        12,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        47,
        4,
        13,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        48,
        4,
        17,
        '{"is_show":"true","can_edit":"true"}',
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        49,
        4,
        18,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        50,
        4,
        7,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        51,
        4,
        8,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        52,
        4,
        9,
        NULL,
        13,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        53,
        4,
        10,
        NULL,
        14,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        62,
        3,
        6,
        '{"statistics_name":"miss_count.miss"}',
        18,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        63,
        5,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        64,
        5,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        65,
        5,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        66,
        5,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        67,
        5,
        5,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        68,
        5,
        6,
        '{"statistics_name":"invalid_length_count.valids"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        69,
        5,
        24,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        70,
        5,
        23,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        71,
        5,
        7,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        72,
        5,
        8,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        73,
        5,
        9,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        74,
        5,
        10,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        75,
        5,
        17,
        '',
        13,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        76,
        5,
        19,
        NULL,
        14,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        79,
        6,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        80,
        6,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        81,
        6,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        82,
        6,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        83,
        6,
        5,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        84,
        6,
        6,
        '{"statistics_name":"duplicate_count.duplicates"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        85,
        6,
        7,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        86,
        6,
        8,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        87,
        6,
        9,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        88,
        6,
        10,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        89,
        6,
        17,
        '',
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        90,
        6,
        19,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        93,
        7,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        94,
        7,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        95,
        7,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        96,
        7,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        97,
        7,
        5,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        98,
        7,
        6,
        '{"statistics_name":"regexp_count.regexps"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        99,
        7,
        25,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        100,
        7,
        7,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        101,
        7,
        8,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        102,
        7,
        9,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        103,
        7,
        10,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        104,
        7,
        17,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        105,
        7,
        19,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        108,
        8,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        109,
        8,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        110,
        8,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        111,
        8,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        112,
        8,
        5,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        113,
        8,
        6,
        '{"statistics_name":"timeliness_count.timeliness"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        114,
        8,
        26,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        115,
        8,
        27,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        116,
        8,
        7,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        117,
        8,
        8,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        118,
        8,
        9,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        119,
        8,
        10,
        NULL,
        13,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        120,
        8,
        17,
        NULL,
        14,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        121,
        8,
        19,
        NULL,
        15,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        124,
        9,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        125,
        9,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        126,
        9,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        127,
        9,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        128,
        9,
        5,
        NULL,
        5,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        129,
        9,
        6,
        '{"statistics_name":"enum_count.enums"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        130,
        9,
        28,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        131,
        9,
        7,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        132,
        9,
        8,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        133,
        9,
        9,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        134,
        9,
        10,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        135,
        9,
        17,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        136,
        9,
        19,
        NULL,
        13,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        139,
        10,
        1,
        NULL,
        1,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        140,
        10,
        2,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        141,
        10,
        3,
        NULL,
        3,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        142,
        10,
        4,
        NULL,
        4,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        143,
        10,
        6,
        '{"statistics_name":"table_count.total"}',
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        144,
        10,
        7,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        145,
        10,
        8,
        NULL,
        8,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        146,
        10,
        9,
        NULL,
        9,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        147,
        10,
        10,
        NULL,
        10,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        148,
        10,
        17,
        NULL,
        11,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        149,
        10,
        19,
        NULL,
        12,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        150,
        8,
        29,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        151,
        1,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        152,
        2,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        153,
        3,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        154,
        4,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        155,
        5,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        156,
        6,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        157,
        7,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        158,
        8,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        159,
        9,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        160,
        10,
        30,
        NULL,
        2,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        161,
        3,
        31,
        NULL,
        6,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

INSERT INTO
    public.t_ds_relation_rule_input_entry
VALUES
    (
        162,
        4,
        31,
        NULL,
        7,
        '2021-03-03 11:31:24',
        '2021-03-03 11:31:24'
    );

--
-- Data for Name: t_ds_relation_sub_workflow; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_relation_udfs_user; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_resources; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_schedules; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_session; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_session
VALUES
    (
        'bad8d8c3-209e-4858-b639-26cf232300ef',
        1,
        '125.35.75.30',
        '2023-11-13 09:55:27.504'
    );

--
-- Data for Name: t_ds_task_definition; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_task_definition_log; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_task_group; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_task_group_queue; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_task_instance; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_tenant; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_tenant
VALUES
    (
        -1,
        'default',
        'default tenant',
        1,
        '2018-03-27 15:48:50',
        '2018-10-24 17:40:22'
    );

--
-- Data for Name: t_ds_trigger_relation; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_udfs; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Data for Name: t_ds_user; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_user
VALUES
    (
        1,
        'admin',
        '7ad2410b2f4c074479a8937a28a22b8f',
        0,
        'xxx@qq.com',
        '',
        -1,
        '2018-03-27 15:48:50',
        '2018-10-24 17:40:22',
        NULL,
        1,
        NULL
    );

--
-- Data for Name: t_ds_version; Type: TABLE DATA; Schema: public; Owner: root
--
INSERT INTO
    public.t_ds_version
VALUES
    (1, '3.2.0');

--
-- Data for Name: t_ds_worker_group; Type: TABLE DATA; Schema: public; Owner: root
--
--
-- Name: t_ds_access_token_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_access_token_id_sequence', 1, false);

--
-- Name: t_ds_alert_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_alert_id_sequence', 1, false);

--
-- Name: t_ds_alert_plugin_instance_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_alert_plugin_instance_id_seq',
        1,
        false
    );

--
-- Name: t_ds_alert_send_status_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_alert_send_status_id_seq', 1, false);

--
-- Name: t_ds_alertgroup_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_alertgroup_id_sequence', 33, true);

--
-- Name: t_ds_audit_log_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_audit_log_id_seq', 1, false);

--
-- Name: t_ds_cluster_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_cluster_id_seq', 1, false);

--
-- Name: t_ds_command_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_command_id_sequence', 1, false);

--
-- Name: t_ds_datasource_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_datasource_id_sequence', 1, false);

--
-- Name: t_ds_dq_comparison_type_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_dq_comparison_type_id_seq',
        1,
        false
    );

--
-- Name: t_ds_dq_execute_result_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_dq_execute_result_id_seq', 1, false);

--
-- Name: t_ds_dq_rule_execute_sql_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_dq_rule_execute_sql_id_seq',
        1,
        false
    );

--
-- Name: t_ds_dq_rule_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_dq_rule_id_seq', 1, false);

--
-- Name: t_ds_dq_rule_input_entry_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_dq_rule_input_entry_id_seq',
        1,
        false
    );

--
-- Name: t_ds_dq_task_statistics_value_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_dq_task_statistics_value_id_seq',
        1,
        false
    );

--
-- Name: t_ds_environment_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_environment_id_seq', 1, false);

--
-- Name: t_ds_environment_worker_group_relation_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_environment_worker_group_relation_id_seq',
        1,
        false
    );

--
-- Name: t_ds_fav_task_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_fav_task_id_seq', 1, false);

--
-- Name: t_ds_k8s_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_k8s_id_seq', 1, false);

--
-- Name: t_ds_k8s_namespace_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_k8s_namespace_id_seq', 1, false);

--
-- Name: t_ds_plugin_define_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_plugin_define_id_seq', 66, true);

--
-- Name: t_ds_process_definition_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_process_definition_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_process_definition_log_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_process_definition_log_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_process_instance_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_process_instance_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_process_task_relation_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_process_task_relation_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_process_task_relation_log_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_process_task_relation_log_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_project_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_project_id_sequence', 11, true);

--
-- Name: t_ds_project_parameter_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_project_parameter_id_sequence',
        1,
        true
    );

--
-- Name: t_ds_project_preference_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_project_preference_id_sequence',
        1,
        true
    );

--
-- Name: t_ds_queue_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_queue_id_sequence', 33, true);

--
-- Name: t_ds_relation_datasource_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_datasource_user_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_relation_namespace_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_namespace_user_id_seq',
        1,
        false
    );

--
-- Name: t_ds_relation_process_instance_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_process_instance_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_relation_project_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_project_user_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_relation_resources_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_resources_user_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_relation_rule_execute_sql_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_rule_execute_sql_id_seq',
        1,
        false
    );

--
-- Name: t_ds_relation_rule_input_entry_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_rule_input_entry_id_seq',
        1,
        false
    );

--
-- Name: t_ds_relation_sub_workflow_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_sub_workflow_id_seq',
        1,
        false
    );

--
-- Name: t_ds_relation_udfs_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_relation_udfs_user_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_resources_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_resources_id_sequence', 1, false);

--
-- Name: t_ds_schedules_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_schedules_id_sequence', 1, false);

--
-- Name: t_ds_task_definition_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_task_definition_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_task_definition_log_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_task_definition_log_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_task_group_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_task_group_id_seq', 1, false);

--
-- Name: t_ds_task_group_queue_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_task_group_queue_id_seq', 1, false);

--
-- Name: t_ds_task_instance_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval(
        'public.t_ds_task_instance_id_sequence',
        1,
        false
    );

--
-- Name: t_ds_tenant_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_tenant_id_sequence', 1, false);

--
-- Name: t_ds_trigger_relation_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_trigger_relation_id_seq', 1, false);

--
-- Name: t_ds_udfs_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_udfs_id_sequence', 1, false);

--
-- Name: t_ds_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_user_id_sequence', 33, true);

--
-- Name: t_ds_version_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_version_id_sequence', 33, true);

--
-- Name: t_ds_worker_group_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--
SELECT
    pg_catalog.setval('public.t_ds_worker_group_id_sequence', 1, false);

--
-- Name: t_ds_alert_send_status alert_send_status_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alert_send_status
ADD
    CONSTRAINT alert_send_status_unique UNIQUE (alert_id, alert_plugin_instance_id);

--
-- Name: t_ds_cluster cluster_code_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_cluster
ADD
    CONSTRAINT cluster_code_unique UNIQUE (code);

--
-- Name: t_ds_cluster cluster_name_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_cluster
ADD
    CONSTRAINT cluster_name_unique UNIQUE (name);

--
-- Name: t_ds_environment environment_code_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment
ADD
    CONSTRAINT environment_code_unique UNIQUE (code);

--
-- Name: t_ds_environment environment_name_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment
ADD
    CONSTRAINT environment_name_unique UNIQUE (name);

--
-- Name: t_ds_environment_worker_group_relation environment_worker_group_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment_worker_group_relation
ADD
    CONSTRAINT environment_worker_group_unique UNIQUE (environment_code, worker_group);

--
-- Name: t_ds_k8s_namespace k8s_namespace_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_k8s_namespace
ADD
    CONSTRAINT k8s_namespace_unique UNIQUE (namespace, cluster_code);

--
-- Name: t_ds_worker_group name_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_worker_group
ADD
    CONSTRAINT name_unique UNIQUE (name);

--
-- Name: t_ds_relation_namespace_user namespace_user_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_namespace_user
ADD
    CONSTRAINT namespace_user_unique UNIQUE (user_id, namespace_id);

--
-- Name: t_ds_process_definition process_definition_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_process_definition
ADD
    CONSTRAINT process_definition_unique UNIQUE (name, project_code);

--
-- Name: qrtz_blob_triggers qrtz_blob_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_blob_triggers
ADD
    CONSTRAINT qrtz_blob_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);

--
-- Name: qrtz_calendars qrtz_calendars_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_calendars
ADD
    CONSTRAINT qrtz_calendars_pkey PRIMARY KEY (sched_name, calendar_name);

--
-- Name: qrtz_cron_triggers qrtz_cron_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_cron_triggers
ADD
    CONSTRAINT qrtz_cron_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);

--
-- Name: qrtz_fired_triggers qrtz_fired_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_fired_triggers
ADD
    CONSTRAINT qrtz_fired_triggers_pkey PRIMARY KEY (sched_name, entry_id);

--
-- Name: qrtz_job_details qrtz_job_details_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_job_details
ADD
    CONSTRAINT qrtz_job_details_pkey PRIMARY KEY (sched_name, job_name, job_group);

--
-- Name: qrtz_locks qrtz_locks_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_locks
ADD
    CONSTRAINT qrtz_locks_pkey PRIMARY KEY (sched_name, lock_name);

--
-- Name: qrtz_paused_trigger_grps qrtz_paused_trigger_grps_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_paused_trigger_grps
ADD
    CONSTRAINT qrtz_paused_trigger_grps_pkey PRIMARY KEY (sched_name, trigger_group);

--
-- Name: qrtz_scheduler_state qrtz_scheduler_state_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_scheduler_state
ADD
    CONSTRAINT qrtz_scheduler_state_pkey PRIMARY KEY (sched_name, instance_name);

--
-- Name: qrtz_simple_triggers qrtz_simple_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_simple_triggers
ADD
    CONSTRAINT qrtz_simple_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);

--
-- Name: qrtz_simprop_triggers qrtz_simprop_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_simprop_triggers
ADD
    CONSTRAINT qrtz_simprop_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);

--
-- Name: qrtz_triggers qrtz_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.qrtz_triggers
ADD
    CONSTRAINT qrtz_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);

--
-- Name: t_ds_access_token t_ds_access_token_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_access_token
ADD
    CONSTRAINT t_ds_access_token_pkey PRIMARY KEY (id);

--
-- Name: t_ds_alert t_ds_alert_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alert
ADD
    CONSTRAINT t_ds_alert_pkey PRIMARY KEY (id);

--
-- Name: t_ds_alert_plugin_instance t_ds_alert_plugin_instance_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alert_plugin_instance
ADD
    CONSTRAINT t_ds_alert_plugin_instance_pk PRIMARY KEY (id);

--
-- Name: t_ds_alert_send_status t_ds_alert_send_status_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alert_send_status
ADD
    CONSTRAINT t_ds_alert_send_status_pkey PRIMARY KEY (id);

--
-- Name: t_ds_alertgroup t_ds_alertgroup_name_un; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alertgroup
ADD
    CONSTRAINT t_ds_alertgroup_name_un UNIQUE (group_name);

--
-- Name: t_ds_alertgroup t_ds_alertgroup_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_alertgroup
ADD
    CONSTRAINT t_ds_alertgroup_pkey PRIMARY KEY (id);

--
-- Name: t_ds_audit_log t_ds_audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_audit_log
ADD
    CONSTRAINT t_ds_audit_log_pkey PRIMARY KEY (id);

--
-- Name: t_ds_cluster t_ds_cluster_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_cluster
ADD
    CONSTRAINT t_ds_cluster_pkey PRIMARY KEY (id);

--
-- Name: t_ds_command t_ds_command_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_command
ADD
    CONSTRAINT t_ds_command_pkey PRIMARY KEY (id);

--
-- Name: t_ds_datasource t_ds_datasource_name_un; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_datasource
ADD
    CONSTRAINT t_ds_datasource_name_un UNIQUE (name, type);

--
-- Name: t_ds_datasource t_ds_datasource_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_datasource
ADD
    CONSTRAINT t_ds_datasource_pkey PRIMARY KEY (id);

--
-- Name: t_ds_dq_comparison_type t_ds_dq_comparison_type_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_comparison_type
ADD
    CONSTRAINT t_ds_dq_comparison_type_pk PRIMARY KEY (id);

--
-- Name: t_ds_dq_execute_result t_ds_dq_execute_result_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_execute_result
ADD
    CONSTRAINT t_ds_dq_execute_result_pk PRIMARY KEY (id);

--
-- Name: t_ds_dq_rule_execute_sql t_ds_dq_rule_execute_sql_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_rule_execute_sql
ADD
    CONSTRAINT t_ds_dq_rule_execute_sql_pk PRIMARY KEY (id);

--
-- Name: t_ds_dq_rule_input_entry t_ds_dq_rule_input_entry_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_rule_input_entry
ADD
    CONSTRAINT t_ds_dq_rule_input_entry_pk PRIMARY KEY (id);

--
-- Name: t_ds_dq_rule t_ds_dq_rule_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_rule
ADD
    CONSTRAINT t_ds_dq_rule_pk PRIMARY KEY (id);

--
-- Name: t_ds_dq_task_statistics_value t_ds_dq_task_statistics_value_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_dq_task_statistics_value
ADD
    CONSTRAINT t_ds_dq_task_statistics_value_pk PRIMARY KEY (id);

--
-- Name: t_ds_environment t_ds_environment_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment
ADD
    CONSTRAINT t_ds_environment_pkey PRIMARY KEY (id);

--
-- Name: t_ds_environment_worker_group_relation t_ds_environment_worker_group_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment_worker_group_relation
ADD
    CONSTRAINT t_ds_environment_worker_group_relation_pkey PRIMARY KEY (id);

--
-- Name: t_ds_error_command t_ds_error_command_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_error_command
ADD
    CONSTRAINT t_ds_error_command_pkey PRIMARY KEY (id);

--
-- Name: t_ds_fav_task t_ds_fav_task_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_fav_task
ADD
    CONSTRAINT t_ds_fav_task_pkey PRIMARY KEY (id);

--
-- Name: t_ds_k8s_namespace t_ds_k8s_namespace_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_k8s_namespace
ADD
    CONSTRAINT t_ds_k8s_namespace_pkey PRIMARY KEY (id);

--
-- Name: t_ds_k8s t_ds_k8s_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_k8s
ADD
    CONSTRAINT t_ds_k8s_pkey PRIMARY KEY (id);

--
-- Name: t_ds_plugin_define t_ds_plugin_define_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_plugin_define
ADD
    CONSTRAINT t_ds_plugin_define_pk PRIMARY KEY (id);

--
-- Name: t_ds_plugin_define t_ds_plugin_define_un; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_plugin_define
ADD
    CONSTRAINT t_ds_plugin_define_un UNIQUE (plugin_name, plugin_type);

--
-- Name: t_ds_process_definition_log t_ds_process_definition_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_process_definition_log
ADD
    CONSTRAINT t_ds_process_definition_log_pkey PRIMARY KEY (id);

--
-- Name: t_ds_process_definition t_ds_process_definition_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_process_definition
ADD
    CONSTRAINT t_ds_process_definition_pkey PRIMARY KEY (id);

--
-- Name: t_ds_process_instance t_ds_process_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_process_instance
ADD
    CONSTRAINT t_ds_process_instance_pkey PRIMARY KEY (id);

--
-- Name: t_ds_process_task_relation_log t_ds_process_task_relation_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_process_task_relation_log
ADD
    CONSTRAINT t_ds_process_task_relation_log_pkey PRIMARY KEY (id);

--
-- Name: t_ds_process_task_relation t_ds_process_task_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_process_task_relation
ADD
    CONSTRAINT t_ds_process_task_relation_pkey PRIMARY KEY (id);

--
-- Name: t_ds_project_parameter t_ds_project_parameter_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_project_parameter
ADD
    CONSTRAINT t_ds_project_parameter_pkey PRIMARY KEY (id);

--
-- Name: t_ds_project t_ds_project_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_project
ADD
    CONSTRAINT t_ds_project_pkey PRIMARY KEY (id);

--
-- Name: t_ds_project_preference t_ds_project_preference_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_project_preference
ADD
    CONSTRAINT t_ds_project_preference_pkey PRIMARY KEY (id);

--
-- Name: t_ds_queue t_ds_queue_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_queue
ADD
    CONSTRAINT t_ds_queue_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_datasource_user t_ds_relation_datasource_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_datasource_user
ADD
    CONSTRAINT t_ds_relation_datasource_user_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_namespace_user t_ds_relation_namespace_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_namespace_user
ADD
    CONSTRAINT t_ds_relation_namespace_user_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_process_instance t_ds_relation_process_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_process_instance
ADD
    CONSTRAINT t_ds_relation_process_instance_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_project_user t_ds_relation_project_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_project_user
ADD
    CONSTRAINT t_ds_relation_project_user_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_project_user t_ds_relation_project_user_un; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_project_user
ADD
    CONSTRAINT t_ds_relation_project_user_un UNIQUE (user_id, project_id);

--
-- Name: t_ds_relation_resources_user t_ds_relation_resources_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_resources_user
ADD
    CONSTRAINT t_ds_relation_resources_user_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_rule_execute_sql t_ds_relation_rule_execute_sql_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_rule_execute_sql
ADD
    CONSTRAINT t_ds_relation_rule_execute_sql_pk PRIMARY KEY (id);

--
-- Name: t_ds_relation_rule_input_entry t_ds_relation_rule_input_entry_pk; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_rule_input_entry
ADD
    CONSTRAINT t_ds_relation_rule_input_entry_pk PRIMARY KEY (id);

--
-- Name: t_ds_relation_sub_workflow t_ds_relation_sub_workflow_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_sub_workflow
ADD
    CONSTRAINT t_ds_relation_sub_workflow_pkey PRIMARY KEY (id);

--
-- Name: t_ds_relation_udfs_user t_ds_relation_udfs_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_relation_udfs_user
ADD
    CONSTRAINT t_ds_relation_udfs_user_pkey PRIMARY KEY (id);

--
-- Name: t_ds_resources t_ds_resources_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_resources
ADD
    CONSTRAINT t_ds_resources_pkey PRIMARY KEY (id);

--
-- Name: t_ds_resources t_ds_resources_un; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_resources
ADD
    CONSTRAINT t_ds_resources_un UNIQUE (full_name, type);

--
-- Name: t_ds_schedules t_ds_schedules_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_schedules
ADD
    CONSTRAINT t_ds_schedules_pkey PRIMARY KEY (id);

--
-- Name: t_ds_session t_ds_session_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_session
ADD
    CONSTRAINT t_ds_session_pkey PRIMARY KEY (id);

--
-- Name: t_ds_task_definition_log t_ds_task_definition_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_definition_log
ADD
    CONSTRAINT t_ds_task_definition_log_pkey PRIMARY KEY (id);

--
-- Name: t_ds_task_definition t_ds_task_definition_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_definition
ADD
    CONSTRAINT t_ds_task_definition_pkey PRIMARY KEY (id);

--
-- Name: t_ds_task_group t_ds_task_group_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_group
ADD
    CONSTRAINT t_ds_task_group_pkey PRIMARY KEY (id);

--
-- Name: t_ds_task_group_queue t_ds_task_group_queue_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_group_queue
ADD
    CONSTRAINT t_ds_task_group_queue_pkey PRIMARY KEY (id);

--
-- Name: t_ds_task_instance t_ds_task_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_task_instance
ADD
    CONSTRAINT t_ds_task_instance_pkey PRIMARY KEY (id);

--
-- Name: t_ds_tenant t_ds_tenant_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_tenant
ADD
    CONSTRAINT t_ds_tenant_pkey PRIMARY KEY (id);

--
-- Name: t_ds_trigger_relation t_ds_trigger_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_trigger_relation
ADD
    CONSTRAINT t_ds_trigger_relation_pkey PRIMARY KEY (id);

--
-- Name: t_ds_trigger_relation t_ds_trigger_relation_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_trigger_relation
ADD
    CONSTRAINT t_ds_trigger_relation_unique UNIQUE (trigger_type, job_id, trigger_code);

--
-- Name: t_ds_udfs t_ds_udfs_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_udfs
ADD
    CONSTRAINT t_ds_udfs_pkey PRIMARY KEY (id);

--
-- Name: t_ds_user t_ds_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_user
ADD
    CONSTRAINT t_ds_user_pkey PRIMARY KEY (id);

--
-- Name: t_ds_version t_ds_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_version
ADD
    CONSTRAINT t_ds_version_pkey PRIMARY KEY (id);
--
-- Name: t_ds_worker_group t_ds_worker_group_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_worker_group
ADD
    CONSTRAINT t_ds_worker_group_pkey PRIMARY KEY (id);