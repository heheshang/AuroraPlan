--
-- PostgreSQL database dump
--

-- Dumped from database version 10.21 (Debian 10.21-1.pgdg90+1)
-- Dumped by pg_dump version 14.8 (Ubuntu 14.8-0ubuntu0.22.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

ALTER TABLE IF EXISTS ONLY public.t_ds_task_instance DROP CONSTRAINT IF EXISTS foreign_key_instance_id;
DROP INDEX IF EXISTS public.version_index;
DROP INDEX IF EXISTS public.user_id_index;
DROP INDEX IF EXISTS public.unique_tenant_code;
DROP INDEX IF EXISTS public.unique_queue_name;
DROP INDEX IF EXISTS public.unique_name;
DROP INDEX IF EXISTS public.unique_func_name;
DROP INDEX IF EXISTS public.unique_code;
DROP INDEX IF EXISTS public.task_definition_index;
DROP INDEX IF EXISTS public.start_time_index;
DROP INDEX IF EXISTS public.relation_project_user_id_index;
DROP INDEX IF EXISTS public.process_task_relation_log_idx_project_code_process_definition_c;
DROP INDEX IF EXISTS public.process_task_relation_idx_project_code_process_definition_code;
DROP INDEX IF EXISTS public.process_task_relation_idx_pre_task_code_version;
DROP INDEX IF EXISTS public.process_task_relation_idx_post_task_code_version;
DROP INDEX IF EXISTS public.process_instance_index;
DROP INDEX IF EXISTS public.process_definition_index;
DROP INDEX IF EXISTS public.priority_id_index;
DROP INDEX IF EXISTS public.idx_task_instance_code_version;
DROP INDEX IF EXISTS public.idx_task_definition_log_project_code;
DROP INDEX IF EXISTS public.idx_task_definition_log_code_version;
DROP INDEX IF EXISTS public.idx_status;
DROP INDEX IF EXISTS public.idx_sign;
DROP INDEX IF EXISTS public.idx_qrtz_t_state;
DROP INDEX IF EXISTS public.idx_qrtz_t_nft_st_misfire_grp;
DROP INDEX IF EXISTS public.idx_qrtz_t_nft_st_misfire;
DROP INDEX IF EXISTS public.idx_qrtz_t_nft_st;
DROP INDEX IF EXISTS public.idx_qrtz_t_nft_misfire;
DROP INDEX IF EXISTS public.idx_qrtz_t_next_fire_time;
DROP INDEX IF EXISTS public.idx_qrtz_t_n_state;
DROP INDEX IF EXISTS public.idx_qrtz_t_n_g_state;
DROP INDEX IF EXISTS public.idx_qrtz_t_jg;
DROP INDEX IF EXISTS public.idx_qrtz_t_j;
DROP INDEX IF EXISTS public.idx_qrtz_t_g;
DROP INDEX IF EXISTS public.idx_qrtz_t_c;
DROP INDEX IF EXISTS public.idx_qrtz_j_req_recovery;
DROP INDEX IF EXISTS public.idx_qrtz_j_grp;
DROP INDEX IF EXISTS public.idx_qrtz_ft_trig_inst_name;
DROP INDEX IF EXISTS public.idx_qrtz_ft_tg;
DROP INDEX IF EXISTS public.idx_qrtz_ft_t_g;
DROP INDEX IF EXISTS public.idx_qrtz_ft_jg;
DROP INDEX IF EXISTS public.idx_qrtz_ft_j_g;
DROP INDEX IF EXISTS public.idx_qrtz_ft_inst_job_req_rcvry;
ALTER TABLE IF EXISTS ONLY public.t_ds_worker_group DROP CONSTRAINT IF EXISTS t_ds_worker_group_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_version DROP CONSTRAINT IF EXISTS t_ds_version_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_user DROP CONSTRAINT IF EXISTS t_ds_user_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_udfs DROP CONSTRAINT IF EXISTS t_ds_udfs_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_tenant DROP CONSTRAINT IF EXISTS t_ds_tenant_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_task_instance DROP CONSTRAINT IF EXISTS t_ds_task_instance_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_task_group_queue DROP CONSTRAINT IF EXISTS t_ds_task_group_queue_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_task_group DROP CONSTRAINT IF EXISTS t_ds_task_group_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_task_definition DROP CONSTRAINT IF EXISTS t_ds_task_definition_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_task_definition_log DROP CONSTRAINT IF EXISTS t_ds_task_definition_log_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_session DROP CONSTRAINT IF EXISTS t_ds_session_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_schedules DROP CONSTRAINT IF EXISTS t_ds_schedules_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_resources DROP CONSTRAINT IF EXISTS t_ds_resources_un;
ALTER TABLE IF EXISTS ONLY public.t_ds_resources DROP CONSTRAINT IF EXISTS t_ds_resources_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_udfs_user DROP CONSTRAINT IF EXISTS t_ds_relation_udfs_user_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_rule_input_entry DROP CONSTRAINT IF EXISTS t_ds_relation_rule_input_entry_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_rule_execute_sql DROP CONSTRAINT IF EXISTS t_ds_relation_rule_execute_sql_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_resources_user DROP CONSTRAINT IF EXISTS t_ds_relation_resources_user_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_project_user DROP CONSTRAINT IF EXISTS t_ds_relation_project_user_un;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_project_user DROP CONSTRAINT IF EXISTS t_ds_relation_project_user_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_process_instance DROP CONSTRAINT IF EXISTS t_ds_relation_process_instance_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_namespace_user DROP CONSTRAINT IF EXISTS t_ds_relation_namespace_user_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_datasource_user DROP CONSTRAINT IF EXISTS t_ds_relation_datasource_user_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_queue DROP CONSTRAINT IF EXISTS t_ds_queue_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_project DROP CONSTRAINT IF EXISTS t_ds_project_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_process_task_relation DROP CONSTRAINT IF EXISTS t_ds_process_task_relation_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_process_task_relation_log DROP CONSTRAINT IF EXISTS t_ds_process_task_relation_log_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_process_instance DROP CONSTRAINT IF EXISTS t_ds_process_instance_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_process_definition DROP CONSTRAINT IF EXISTS t_ds_process_definition_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_process_definition_log DROP CONSTRAINT IF EXISTS t_ds_process_definition_log_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_plugin_define DROP CONSTRAINT IF EXISTS t_ds_plugin_define_un;
ALTER TABLE IF EXISTS ONLY public.t_ds_plugin_define DROP CONSTRAINT IF EXISTS t_ds_plugin_define_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_k8s DROP CONSTRAINT IF EXISTS t_ds_k8s_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_k8s_namespace DROP CONSTRAINT IF EXISTS t_ds_k8s_namespace_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_error_command DROP CONSTRAINT IF EXISTS t_ds_error_command_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_environment_worker_group_relation DROP CONSTRAINT IF EXISTS t_ds_environment_worker_group_relation_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_environment DROP CONSTRAINT IF EXISTS t_ds_environment_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_dq_task_statistics_value DROP CONSTRAINT IF EXISTS t_ds_dq_task_statistics_value_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_dq_rule DROP CONSTRAINT IF EXISTS t_ds_dq_rule_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_dq_rule_input_entry DROP CONSTRAINT IF EXISTS t_ds_dq_rule_input_entry_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_dq_rule_execute_sql DROP CONSTRAINT IF EXISTS t_ds_dq_rule_execute_sql_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_dq_execute_result DROP CONSTRAINT IF EXISTS t_ds_dq_execute_result_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_dq_comparison_type DROP CONSTRAINT IF EXISTS t_ds_dq_comparison_type_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_datasource DROP CONSTRAINT IF EXISTS t_ds_datasource_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_datasource DROP CONSTRAINT IF EXISTS t_ds_datasource_name_un;
ALTER TABLE IF EXISTS ONLY public.t_ds_command DROP CONSTRAINT IF EXISTS t_ds_command_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_audit_log DROP CONSTRAINT IF EXISTS t_ds_audit_log_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_alertgroup DROP CONSTRAINT IF EXISTS t_ds_alertgroup_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_alertgroup DROP CONSTRAINT IF EXISTS t_ds_alertgroup_name_un;
ALTER TABLE IF EXISTS ONLY public.t_ds_alert_send_status DROP CONSTRAINT IF EXISTS t_ds_alert_send_status_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_alert_plugin_instance DROP CONSTRAINT IF EXISTS t_ds_alert_plugin_instance_pk;
ALTER TABLE IF EXISTS ONLY public.t_ds_alert DROP CONSTRAINT IF EXISTS t_ds_alert_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_access_token DROP CONSTRAINT IF EXISTS t_ds_access_token_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_triggers DROP CONSTRAINT IF EXISTS qrtz_triggers_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_simprop_triggers DROP CONSTRAINT IF EXISTS qrtz_simprop_triggers_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_simple_triggers DROP CONSTRAINT IF EXISTS qrtz_simple_triggers_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_scheduler_state DROP CONSTRAINT IF EXISTS qrtz_scheduler_state_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_paused_trigger_grps DROP CONSTRAINT IF EXISTS qrtz_paused_trigger_grps_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_locks DROP CONSTRAINT IF EXISTS qrtz_locks_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_job_details DROP CONSTRAINT IF EXISTS qrtz_job_details_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_fired_triggers DROP CONSTRAINT IF EXISTS qrtz_fired_triggers_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_cron_triggers DROP CONSTRAINT IF EXISTS qrtz_cron_triggers_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_calendars DROP CONSTRAINT IF EXISTS qrtz_calendars_pkey;
ALTER TABLE IF EXISTS ONLY public.qrtz_blob_triggers DROP CONSTRAINT IF EXISTS qrtz_blob_triggers_pkey;
ALTER TABLE IF EXISTS ONLY public.t_ds_process_definition DROP CONSTRAINT IF EXISTS process_definition_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_relation_namespace_user DROP CONSTRAINT IF EXISTS namespace_user_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_worker_group DROP CONSTRAINT IF EXISTS name_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_k8s_namespace DROP CONSTRAINT IF EXISTS k8s_namespace_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_environment_worker_group_relation DROP CONSTRAINT IF EXISTS environment_worker_group_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_environment DROP CONSTRAINT IF EXISTS environment_name_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_environment DROP CONSTRAINT IF EXISTS environment_code_unique;
ALTER TABLE IF EXISTS ONLY public.t_ds_alert_send_status DROP CONSTRAINT IF EXISTS alert_send_status_unique;
ALTER TABLE IF EXISTS public.t_ds_task_group_queue ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_task_group ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_relation_rule_input_entry ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_relation_rule_execute_sql ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_relation_namespace_user ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_plugin_define ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_k8s_namespace ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_k8s ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_environment_worker_group_relation ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_environment ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_dq_task_statistics_value ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_dq_rule_input_entry ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_dq_rule_execute_sql ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_dq_rule ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_dq_execute_result ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_dq_comparison_type ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_audit_log ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_alert_send_status ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS public.t_ds_alert_plugin_instance ALTER COLUMN id DROP DEFAULT;
DROP TABLE IF EXISTS public.t_ds_worker_group;
DROP SEQUENCE IF EXISTS public.t_ds_worker_group_id_sequence;
DROP TABLE IF EXISTS public.t_ds_version;
DROP SEQUENCE IF EXISTS public.t_ds_version_id_sequence;
DROP TABLE IF EXISTS public.t_ds_user;
DROP SEQUENCE IF EXISTS public.t_ds_user_id_sequence;
DROP TABLE IF EXISTS public.t_ds_udfs;
DROP SEQUENCE IF EXISTS public.t_ds_udfs_id_sequence;
DROP TABLE IF EXISTS public.t_ds_tenant;
DROP SEQUENCE IF EXISTS public.t_ds_tenant_id_sequence;
DROP TABLE IF EXISTS public.t_ds_task_instance;
DROP SEQUENCE IF EXISTS public.t_ds_task_instance_id_sequence;
DROP SEQUENCE IF EXISTS public.t_ds_task_group_queue_id_seq;
DROP TABLE IF EXISTS public.t_ds_task_group_queue;
DROP SEQUENCE IF EXISTS public.t_ds_task_group_id_seq;
DROP TABLE IF EXISTS public.t_ds_task_group;
DROP TABLE IF EXISTS public.t_ds_task_definition_log;
DROP SEQUENCE IF EXISTS public.t_ds_task_definition_log_id_sequence;
DROP TABLE IF EXISTS public.t_ds_task_definition;
DROP SEQUENCE IF EXISTS public.t_ds_task_definition_id_sequence;
DROP TABLE IF EXISTS public.t_ds_session;
DROP TABLE IF EXISTS public.t_ds_schedules;
DROP SEQUENCE IF EXISTS public.t_ds_schedules_id_sequence;
DROP TABLE IF EXISTS public.t_ds_resources;
DROP SEQUENCE IF EXISTS public.t_ds_resources_id_sequence;
DROP TABLE IF EXISTS public.t_ds_relation_udfs_user;
DROP SEQUENCE IF EXISTS public.t_ds_relation_udfs_user_id_sequence;
DROP SEQUENCE IF EXISTS public.t_ds_relation_rule_input_entry_id_seq;
DROP TABLE IF EXISTS public.t_ds_relation_rule_input_entry;
DROP SEQUENCE IF EXISTS public.t_ds_relation_rule_execute_sql_id_seq;
DROP TABLE IF EXISTS public.t_ds_relation_rule_execute_sql;
DROP TABLE IF EXISTS public.t_ds_relation_resources_user;
DROP SEQUENCE IF EXISTS public.t_ds_relation_resources_user_id_sequence;
DROP TABLE IF EXISTS public.t_ds_relation_project_user;
DROP SEQUENCE IF EXISTS public.t_ds_relation_project_user_id_sequence;
DROP TABLE IF EXISTS public.t_ds_relation_process_instance;
DROP SEQUENCE IF EXISTS public.t_ds_relation_process_instance_id_sequence;
DROP SEQUENCE IF EXISTS public.t_ds_relation_namespace_user_id_seq;
DROP TABLE IF EXISTS public.t_ds_relation_namespace_user;
DROP TABLE IF EXISTS public.t_ds_relation_datasource_user;
DROP SEQUENCE IF EXISTS public.t_ds_relation_datasource_user_id_sequence;
DROP TABLE IF EXISTS public.t_ds_queue;
DROP SEQUENCE IF EXISTS public.t_ds_queue_id_sequence;
DROP TABLE IF EXISTS public.t_ds_project;
DROP SEQUENCE IF EXISTS public.t_ds_project_id_sequence;
DROP TABLE IF EXISTS public.t_ds_process_task_relation_log;
DROP SEQUENCE IF EXISTS public.t_ds_process_task_relation_log_id_sequence;
DROP TABLE IF EXISTS public.t_ds_process_task_relation;
DROP SEQUENCE IF EXISTS public.t_ds_process_task_relation_id_sequence;
DROP TABLE IF EXISTS public.t_ds_process_instance;
DROP SEQUENCE IF EXISTS public.t_ds_process_instance_id_sequence;
DROP TABLE IF EXISTS public.t_ds_process_definition_log;
DROP SEQUENCE IF EXISTS public.t_ds_process_definition_log_id_sequence;
DROP TABLE IF EXISTS public.t_ds_process_definition;
DROP SEQUENCE IF EXISTS public.t_ds_process_definition_id_sequence;
DROP SEQUENCE IF EXISTS public.t_ds_plugin_define_id_seq;
DROP TABLE IF EXISTS public.t_ds_plugin_define;
DROP SEQUENCE IF EXISTS public.t_ds_k8s_namespace_id_seq;
DROP TABLE IF EXISTS public.t_ds_k8s_namespace;
DROP SEQUENCE IF EXISTS public.t_ds_k8s_id_seq;
DROP TABLE IF EXISTS public.t_ds_k8s;
DROP TABLE IF EXISTS public.t_ds_error_command;
DROP SEQUENCE IF EXISTS public.t_ds_environment_worker_group_relation_id_seq;
DROP TABLE IF EXISTS public.t_ds_environment_worker_group_relation;
DROP SEQUENCE IF EXISTS public.t_ds_environment_id_seq;
DROP TABLE IF EXISTS public.t_ds_environment;
DROP SEQUENCE IF EXISTS public.t_ds_dq_task_statistics_value_id_seq;
DROP TABLE IF EXISTS public.t_ds_dq_task_statistics_value;
DROP SEQUENCE IF EXISTS public.t_ds_dq_rule_input_entry_id_seq;
DROP TABLE IF EXISTS public.t_ds_dq_rule_input_entry;
DROP SEQUENCE IF EXISTS public.t_ds_dq_rule_id_seq;
DROP SEQUENCE IF EXISTS public.t_ds_dq_rule_execute_sql_id_seq;
DROP TABLE IF EXISTS public.t_ds_dq_rule_execute_sql;
DROP TABLE IF EXISTS public.t_ds_dq_rule;
DROP SEQUENCE IF EXISTS public.t_ds_dq_execute_result_id_seq;
DROP TABLE IF EXISTS public.t_ds_dq_execute_result;
DROP SEQUENCE IF EXISTS public.t_ds_dq_comparison_type_id_seq;
DROP TABLE IF EXISTS public.t_ds_dq_comparison_type;
DROP TABLE IF EXISTS public.t_ds_datasource;
DROP SEQUENCE IF EXISTS public.t_ds_datasource_id_sequence;
DROP TABLE IF EXISTS public.t_ds_command;
DROP SEQUENCE IF EXISTS public.t_ds_command_id_sequence;
DROP SEQUENCE IF EXISTS public.t_ds_audit_log_id_seq;
DROP TABLE IF EXISTS public.t_ds_audit_log;
DROP TABLE IF EXISTS public.t_ds_alertgroup;
DROP SEQUENCE IF EXISTS public.t_ds_alertgroup_id_sequence;
DROP SEQUENCE IF EXISTS public.t_ds_alert_send_status_id_seq;
DROP TABLE IF EXISTS public.t_ds_alert_send_status;
DROP SEQUENCE IF EXISTS public.t_ds_alert_plugin_instance_id_seq;
DROP TABLE IF EXISTS public.t_ds_alert_plugin_instance;
DROP TABLE IF EXISTS public.t_ds_alert;
DROP SEQUENCE IF EXISTS public.t_ds_alert_id_sequence;
DROP TABLE IF EXISTS public.t_ds_access_token;
DROP SEQUENCE IF EXISTS public.t_ds_access_token_id_sequence;
DROP TABLE IF EXISTS public.qrtz_triggers;
DROP TABLE IF EXISTS public.qrtz_simprop_triggers;
DROP TABLE IF EXISTS public.qrtz_simple_triggers;
DROP TABLE IF EXISTS public.qrtz_scheduler_state;
DROP TABLE IF EXISTS public.qrtz_paused_trigger_grps;
DROP TABLE IF EXISTS public.qrtz_locks;
DROP TABLE IF EXISTS public.qrtz_job_details;
DROP TABLE IF EXISTS public.qrtz_fired_triggers;
DROP TABLE IF EXISTS public.qrtz_cron_triggers;
DROP TABLE IF EXISTS public.qrtz_calendars;
DROP TABLE IF EXISTS public.qrtz_blob_triggers;
SET default_tablespace = '';

--
-- Name: qrtz_blob_triggers; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.qrtz_blob_triggers (
    sched_name character varying(120) NOT NULL,
    trigger_name character varying(200) NOT NULL,
    trigger_group character varying(200) NOT NULL,
    blob_data bytea
);


ALTER TABLE public.qrtz_blob_triggers OWNER TO root;

--
-- Name: qrtz_calendars; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.qrtz_calendars (
    sched_name character varying(120) NOT NULL,
    calendar_name character varying(200) NOT NULL,
    calendar bytea NOT NULL
);


ALTER TABLE public.qrtz_calendars OWNER TO root;

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


ALTER TABLE public.qrtz_cron_triggers OWNER TO root;

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


ALTER TABLE public.qrtz_fired_triggers OWNER TO root;

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


ALTER TABLE public.qrtz_job_details OWNER TO root;

--
-- Name: qrtz_locks; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.qrtz_locks (
    sched_name character varying(120) NOT NULL,
    lock_name character varying(40) NOT NULL
);


ALTER TABLE public.qrtz_locks OWNER TO root;

--
-- Name: qrtz_paused_trigger_grps; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.qrtz_paused_trigger_grps (
    sched_name character varying(120) NOT NULL,
    trigger_group character varying(200) NOT NULL
);


ALTER TABLE public.qrtz_paused_trigger_grps OWNER TO root;

--
-- Name: qrtz_scheduler_state; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.qrtz_scheduler_state (
    sched_name character varying(120) NOT NULL,
    instance_name character varying(200) NOT NULL,
    last_checkin_time bigint NOT NULL,
    checkin_interval bigint NOT NULL
);


ALTER TABLE public.qrtz_scheduler_state OWNER TO root;

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


ALTER TABLE public.qrtz_simple_triggers OWNER TO root;

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
    dec_prop_1 numeric(13,4),
    dec_prop_2 numeric(13,4),
    bool_prop_1 boolean,
    bool_prop_2 boolean
);


ALTER TABLE public.qrtz_simprop_triggers OWNER TO root;

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


ALTER TABLE public.qrtz_triggers OWNER TO root;

--
-- Name: t_ds_access_token_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_access_token_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_access_token_id_sequence OWNER TO root;

--
-- Name: t_ds_access_token; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_access_token (
    id integer DEFAULT nextval('public.t_ds_access_token_id_sequence'::regclass) NOT NULL,
    user_id integer,
    token character varying(64) DEFAULT NULL::character varying,
    expire_time timestamp without time zone,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_access_token OWNER TO root;

--
-- Name: t_ds_alert_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_alert_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_alert_id_sequence OWNER TO root;

--
-- Name: t_ds_alert; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_alert (
    id integer DEFAULT nextval('public.t_ds_alert_id_sequence'::regclass) NOT NULL,
    title character varying(64) DEFAULT NULL::character varying,
    sign character varying(40) DEFAULT ''::character varying NOT NULL,
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


ALTER TABLE public.t_ds_alert OWNER TO root;

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
    instance_name character varying(200)
);


ALTER TABLE public.t_ds_alert_plugin_instance OWNER TO root;

--
-- Name: t_ds_alert_plugin_instance_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_alert_plugin_instance_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_alert_plugin_instance_id_seq OWNER TO root;

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


ALTER TABLE public.t_ds_alert_send_status OWNER TO root;

--
-- Name: t_ds_alert_send_status_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_alert_send_status_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_alert_send_status_id_seq OWNER TO root;

--
-- Name: t_ds_alert_send_status_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_alert_send_status_id_seq OWNED BY public.t_ds_alert_send_status.id;


--
-- Name: t_ds_alertgroup_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_alertgroup_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_alertgroup_id_sequence OWNER TO root;

--
-- Name: t_ds_alertgroup; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_alertgroup (
    id integer DEFAULT nextval('public.t_ds_alertgroup_id_sequence'::regclass) NOT NULL,
    alert_instance_ids character varying(255) DEFAULT NULL::character varying,
    create_user_id integer,
    group_name character varying(255) DEFAULT NULL::character varying,
    description character varying(255) DEFAULT NULL::character varying,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_alertgroup OWNER TO root;

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


ALTER TABLE public.t_ds_audit_log OWNER TO root;

--
-- Name: t_ds_audit_log_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_audit_log_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_audit_log_id_seq OWNER TO root;

--
-- Name: t_ds_audit_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_audit_log_id_seq OWNED BY public.t_ds_audit_log.id;


--
-- Name: t_ds_command_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_command_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_command_id_sequence OWNER TO root;

--
-- Name: t_ds_command; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_command (
    id integer DEFAULT nextval('public.t_ds_command_id_sequence'::regclass) NOT NULL,
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
    worker_group character varying(64),
    environment_code bigint DEFAULT '-1'::bigint,
    dry_run integer DEFAULT 0,
    process_instance_id integer DEFAULT 0,
    process_definition_version integer DEFAULT 0
);


ALTER TABLE public.t_ds_command OWNER TO root;

--
-- Name: t_ds_datasource_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_datasource_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_datasource_id_sequence OWNER TO root;

--
-- Name: t_ds_datasource; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_datasource (
    id integer DEFAULT nextval('public.t_ds_datasource_id_sequence'::regclass) NOT NULL,
    name character varying(64) NOT NULL,
    note character varying(255) DEFAULT NULL::character varying,
    type integer NOT NULL,
    user_id integer NOT NULL,
    connection_params text NOT NULL,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_datasource OWNER TO root;

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


ALTER TABLE public.t_ds_dq_comparison_type OWNER TO root;

--
-- Name: t_ds_dq_comparison_type_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_dq_comparison_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_dq_comparison_type_id_seq OWNER TO root;

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
    rule_name character varying(255) DEFAULT NULL::character varying,
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


ALTER TABLE public.t_ds_dq_execute_result OWNER TO root;

--
-- Name: t_ds_dq_execute_result_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_dq_execute_result_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_dq_execute_result_id_seq OWNER TO root;

--
-- Name: t_ds_dq_execute_result_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_dq_execute_result_id_seq OWNED BY public.t_ds_dq_execute_result.id;


--
-- Name: t_ds_dq_rule; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_dq_rule (
    id integer NOT NULL,
    name character varying(100) DEFAULT NULL::character varying,
    type integer,
    user_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_dq_rule OWNER TO root;

--
-- Name: t_ds_dq_rule_execute_sql; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_dq_rule_execute_sql (
    id integer NOT NULL,
    index integer,
    sql text,
    table_alias character varying(255) DEFAULT NULL::character varying,
    type integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    is_error_output_sql boolean
);


ALTER TABLE public.t_ds_dq_rule_execute_sql OWNER TO root;

--
-- Name: t_ds_dq_rule_execute_sql_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_dq_rule_execute_sql_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_dq_rule_execute_sql_id_seq OWNER TO root;

--
-- Name: t_ds_dq_rule_execute_sql_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_dq_rule_execute_sql_id_seq OWNED BY public.t_ds_dq_rule_execute_sql.id;


--
-- Name: t_ds_dq_rule_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_dq_rule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_dq_rule_id_seq OWNER TO root;

--
-- Name: t_ds_dq_rule_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_dq_rule_id_seq OWNED BY public.t_ds_dq_rule.id;


--
-- Name: t_ds_dq_rule_input_entry; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_dq_rule_input_entry (
    id integer NOT NULL,
    field character varying(255) DEFAULT NULL::character varying,
    type character varying(255) DEFAULT NULL::character varying,
    title character varying(255) DEFAULT NULL::character varying,
    value character varying(255) DEFAULT NULL::character varying,
    options text,
    placeholder character varying(255) DEFAULT NULL::character varying,
    option_source_type integer,
    value_type integer,
    input_type integer,
    is_show smallint DEFAULT '1'::smallint,
    can_edit smallint DEFAULT '1'::smallint,
    is_emit smallint DEFAULT '0'::smallint,
    is_validate smallint DEFAULT '0'::smallint,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_dq_rule_input_entry OWNER TO root;

--
-- Name: t_ds_dq_rule_input_entry_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_dq_rule_input_entry_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_dq_rule_input_entry_id_seq OWNER TO root;

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


ALTER TABLE public.t_ds_dq_task_statistics_value OWNER TO root;

--
-- Name: t_ds_dq_task_statistics_value_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_dq_task_statistics_value_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_dq_task_statistics_value_id_seq OWNER TO root;

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
    name character varying(100) DEFAULT NULL::character varying,
    config text,
    description text,
    operator integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_environment OWNER TO root;

--
-- Name: t_ds_environment_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_environment_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_environment_id_seq OWNER TO root;

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


ALTER TABLE public.t_ds_environment_worker_group_relation OWNER TO root;

--
-- Name: t_ds_environment_worker_group_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_environment_worker_group_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_environment_worker_group_relation_id_seq OWNER TO root;

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
    worker_group character varying(64),
    environment_code bigint DEFAULT '-1'::bigint,
    dry_run integer DEFAULT 0,
    message text,
    process_instance_id integer DEFAULT 0,
    process_definition_version integer DEFAULT 0
);


ALTER TABLE public.t_ds_error_command OWNER TO root;

--
-- Name: t_ds_k8s; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_k8s (
    id integer NOT NULL,
    k8s_name character varying(100) DEFAULT NULL::character varying,
    k8s_config text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_k8s OWNER TO root;

--
-- Name: t_ds_k8s_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_k8s_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_k8s_id_seq OWNER TO root;

--
-- Name: t_ds_k8s_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_k8s_id_seq OWNED BY public.t_ds_k8s.id;


--
-- Name: t_ds_k8s_namespace; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_k8s_namespace (
    id integer NOT NULL,
    limits_memory integer,
    namespace character varying(100) DEFAULT NULL::character varying,
    online_job_num integer DEFAULT 0,
    user_id integer,
    pod_replicas integer,
    pod_request_cpu numeric(13,4),
    pod_request_memory integer,
    limits_cpu numeric(13,4),
    k8s character varying(100) DEFAULT NULL::character varying,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_k8s_namespace OWNER TO root;

--
-- Name: t_ds_k8s_namespace_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_k8s_namespace_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_k8s_namespace_id_seq OWNER TO root;

--
-- Name: t_ds_k8s_namespace_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_k8s_namespace_id_seq OWNED BY public.t_ds_k8s_namespace.id;


--
-- Name: t_ds_plugin_define; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_plugin_define (
    id integer NOT NULL,
    plugin_name character varying(100) NOT NULL,
    plugin_type character varying(100) NOT NULL,
    plugin_params text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_plugin_define OWNER TO root;

--
-- Name: t_ds_plugin_define_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_plugin_define_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_plugin_define_id_seq OWNER TO root;

--
-- Name: t_ds_plugin_define_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_plugin_define_id_seq OWNED BY public.t_ds_plugin_define.id;


--
-- Name: t_ds_process_definition_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_process_definition_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_process_definition_id_sequence OWNER TO root;

--
-- Name: t_ds_process_definition; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_process_definition (
    id integer DEFAULT nextval('public.t_ds_process_definition_id_sequence'::regclass) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
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
    tenant_id integer DEFAULT '-1'::integer,
    execution_type integer DEFAULT 0,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_process_definition OWNER TO root;

--
-- Name: t_ds_process_definition_log_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_process_definition_log_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_process_definition_log_id_sequence OWNER TO root;

--
-- Name: t_ds_process_definition_log; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_process_definition_log (
    id integer DEFAULT nextval('public.t_ds_process_definition_log_id_sequence'::regclass) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
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
    tenant_id integer DEFAULT '-1'::integer,
    execution_type integer DEFAULT 0,
    operator integer,
    operate_time timestamp without time zone,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_process_definition_log OWNER TO root;

--
-- Name: t_ds_process_instance_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_process_instance_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_process_instance_id_sequence OWNER TO root;

--
-- Name: t_ds_process_instance; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_process_instance (
    id integer DEFAULT nextval('public.t_ds_process_instance_id_sequence'::regclass) NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
    process_definition_code bigint,
    process_definition_version integer,
    state integer,
    recovery integer,
    start_time timestamp without time zone,
    end_time timestamp without time zone,
    run_times integer,
    host character varying(135) DEFAULT NULL::character varying,
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
    history_cmd text,
    dependence_schedule_times text,
    process_instance_priority integer DEFAULT 2,
    worker_group character varying(64),
    environment_code bigint DEFAULT '-1'::bigint,
    timeout integer DEFAULT 0,
    tenant_id integer DEFAULT '-1'::integer NOT NULL,
    var_pool text,
    dry_run integer DEFAULT 0,
    next_process_instance_id integer DEFAULT 0,
    restart_time timestamp without time zone
);


ALTER TABLE public.t_ds_process_instance OWNER TO root;

--
-- Name: t_ds_process_task_relation_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_process_task_relation_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_process_task_relation_id_sequence OWNER TO root;

--
-- Name: t_ds_process_task_relation; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_process_task_relation (
    id integer DEFAULT nextval('public.t_ds_process_task_relation_id_sequence'::regclass) NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
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


ALTER TABLE public.t_ds_process_task_relation OWNER TO root;

--
-- Name: t_ds_process_task_relation_log_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_process_task_relation_log_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_process_task_relation_log_id_sequence OWNER TO root;

--
-- Name: t_ds_process_task_relation_log; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_process_task_relation_log (
    id integer DEFAULT nextval('public.t_ds_process_task_relation_log_id_sequence'::regclass) NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
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


ALTER TABLE public.t_ds_process_task_relation_log OWNER TO root;

--
-- Name: t_ds_project_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_project_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_project_id_sequence OWNER TO root;

--
-- Name: t_ds_project; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_project (
    id integer DEFAULT nextval('public.t_ds_project_id_sequence'::regclass) NOT NULL,
    name character varying(100) DEFAULT NULL::character varying,
    code bigint NOT NULL,
    description character varying(200) DEFAULT NULL::character varying,
    user_id integer,
    flag integer DEFAULT 1,
    create_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    update_time timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.t_ds_project OWNER TO root;

--
-- Name: t_ds_queue_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_queue_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_queue_id_sequence OWNER TO root;

--
-- Name: t_ds_queue; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_queue (
    id integer DEFAULT nextval('public.t_ds_queue_id_sequence'::regclass) NOT NULL,
    queue_name character varying(64) DEFAULT NULL::character varying,
    queue character varying(64) DEFAULT NULL::character varying,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_queue OWNER TO root;

--
-- Name: t_ds_relation_datasource_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_datasource_user_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_datasource_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_datasource_user; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_relation_datasource_user (
    id integer DEFAULT nextval('public.t_ds_relation_datasource_user_id_sequence'::regclass) NOT NULL,
    user_id integer NOT NULL,
    datasource_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_relation_datasource_user OWNER TO root;

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


ALTER TABLE public.t_ds_relation_namespace_user OWNER TO root;

--
-- Name: t_ds_relation_namespace_user_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_namespace_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_namespace_user_id_seq OWNER TO root;

--
-- Name: t_ds_relation_namespace_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_relation_namespace_user_id_seq OWNED BY public.t_ds_relation_namespace_user.id;


--
-- Name: t_ds_relation_process_instance_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_process_instance_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_process_instance_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_process_instance; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_relation_process_instance (
    id integer DEFAULT nextval('public.t_ds_relation_process_instance_id_sequence'::regclass) NOT NULL,
    parent_process_instance_id integer,
    parent_task_instance_id integer,
    process_instance_id integer
);


ALTER TABLE public.t_ds_relation_process_instance OWNER TO root;

--
-- Name: t_ds_relation_project_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_project_user_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_project_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_project_user; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_relation_project_user (
    id integer DEFAULT nextval('public.t_ds_relation_project_user_id_sequence'::regclass) NOT NULL,
    user_id integer NOT NULL,
    project_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_relation_project_user OWNER TO root;

--
-- Name: t_ds_relation_resources_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_resources_user_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_resources_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_resources_user; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_relation_resources_user (
    id integer DEFAULT nextval('public.t_ds_relation_resources_user_id_sequence'::regclass) NOT NULL,
    user_id integer NOT NULL,
    resources_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_relation_resources_user OWNER TO root;

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


ALTER TABLE public.t_ds_relation_rule_execute_sql OWNER TO root;

--
-- Name: t_ds_relation_rule_execute_sql_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_rule_execute_sql_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_rule_execute_sql_id_seq OWNER TO root;

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


ALTER TABLE public.t_ds_relation_rule_input_entry OWNER TO root;

--
-- Name: t_ds_relation_rule_input_entry_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_rule_input_entry_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_rule_input_entry_id_seq OWNER TO root;

--
-- Name: t_ds_relation_rule_input_entry_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_relation_rule_input_entry_id_seq OWNED BY public.t_ds_relation_rule_input_entry.id;


--
-- Name: t_ds_relation_udfs_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_relation_udfs_user_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_relation_udfs_user_id_sequence OWNER TO root;

--
-- Name: t_ds_relation_udfs_user; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_relation_udfs_user (
    id integer DEFAULT nextval('public.t_ds_relation_udfs_user_id_sequence'::regclass) NOT NULL,
    user_id integer NOT NULL,
    udf_id integer,
    perm integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_relation_udfs_user OWNER TO root;

--
-- Name: t_ds_resources_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_resources_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_resources_id_sequence OWNER TO root;

--
-- Name: t_ds_resources; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_resources (
    id integer DEFAULT nextval('public.t_ds_resources_id_sequence'::regclass) NOT NULL,
    alias character varying(64) DEFAULT NULL::character varying,
    file_name character varying(64) DEFAULT NULL::character varying,
    description character varying(255) DEFAULT NULL::character varying,
    user_id integer,
    type integer,
    size bigint,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    pid integer,
    full_name character varying(128),
    is_directory boolean DEFAULT false
);


ALTER TABLE public.t_ds_resources OWNER TO root;

--
-- Name: t_ds_schedules_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_schedules_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_schedules_id_sequence OWNER TO root;

--
-- Name: t_ds_schedules; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_schedules (
    id integer DEFAULT nextval('public.t_ds_schedules_id_sequence'::regclass) NOT NULL,
    process_definition_code bigint NOT NULL,
    start_time timestamp without time zone NOT NULL,
    end_time timestamp without time zone NOT NULL,
    timezone_id character varying(40) DEFAULT NULL::character varying,
    crontab character varying(255) NOT NULL,
    failure_strategy integer NOT NULL,
    user_id integer NOT NULL,
    release_state integer NOT NULL,
    warning_type integer NOT NULL,
    warning_group_id integer,
    process_instance_priority integer DEFAULT 2,
    worker_group character varying(64),
    environment_code bigint DEFAULT '-1'::bigint,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone NOT NULL
);


ALTER TABLE public.t_ds_schedules OWNER TO root;

--
-- Name: t_ds_session; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_session (
    id character varying(64) NOT NULL,
    user_id integer,
    ip character varying(45) DEFAULT NULL::character varying,
    last_login_time timestamp without time zone
);


ALTER TABLE public.t_ds_session OWNER TO root;

--
-- Name: t_ds_task_definition_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_task_definition_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_task_definition_id_sequence OWNER TO root;

--
-- Name: t_ds_task_definition; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_task_definition (
    id integer DEFAULT nextval('public.t_ds_task_definition_id_sequence'::regclass) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
    version integer NOT NULL,
    description text,
    project_code bigint,
    user_id integer,
    task_type character varying(50) DEFAULT NULL::character varying,
    task_params text,
    flag integer,
    task_priority integer DEFAULT 2,
    worker_group character varying(255) DEFAULT NULL::character varying,
    environment_code bigint DEFAULT '-1'::bigint,
    fail_retry_times integer,
    fail_retry_interval integer,
    timeout_flag integer,
    timeout_notify_strategy integer,
    timeout integer DEFAULT 0,
    delay_time integer DEFAULT 0,
    task_group_id integer,
    task_group_priority integer DEFAULT 0,
    resource_ids text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_task_definition OWNER TO root;

--
-- Name: t_ds_task_definition_log_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_task_definition_log_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_task_definition_log_id_sequence OWNER TO root;

--
-- Name: t_ds_task_definition_log; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_task_definition_log (
    id integer DEFAULT nextval('public.t_ds_task_definition_log_id_sequence'::regclass) NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
    version integer NOT NULL,
    description text,
    project_code bigint,
    user_id integer,
    task_type character varying(50) DEFAULT NULL::character varying,
    task_params text,
    flag integer,
    task_priority integer DEFAULT 2,
    worker_group character varying(255) DEFAULT NULL::character varying,
    environment_code bigint DEFAULT '-1'::bigint,
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
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_task_definition_log OWNER TO root;

--
-- Name: t_ds_task_group; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_task_group (
    id integer NOT NULL,
    name character varying(100) DEFAULT NULL::character varying,
    description character varying(200) DEFAULT NULL::character varying,
    group_size integer NOT NULL,
    project_code bigint DEFAULT '0'::bigint,
    use_size integer DEFAULT 0,
    user_id integer,
    status integer DEFAULT 1,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_task_group OWNER TO root;

--
-- Name: t_ds_task_group_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_task_group_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_task_group_id_seq OWNER TO root;

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
    task_name character varying(100) DEFAULT NULL::character varying,
    group_id integer,
    process_id integer,
    priority integer DEFAULT 0,
    status integer DEFAULT '-1'::integer,
    force_start integer DEFAULT 0,
    in_queue integer DEFAULT 0,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_task_group_queue OWNER TO root;

--
-- Name: t_ds_task_group_queue_id_seq; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_task_group_queue_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_task_group_queue_id_seq OWNER TO root;

--
-- Name: t_ds_task_group_queue_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: root
--

ALTER SEQUENCE public.t_ds_task_group_queue_id_seq OWNED BY public.t_ds_task_group_queue.id;


--
-- Name: t_ds_task_instance_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_task_instance_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_task_instance_id_sequence OWNER TO root;

--
-- Name: t_ds_task_instance; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_task_instance (
    id integer DEFAULT nextval('public.t_ds_task_instance_id_sequence'::regclass) NOT NULL,
    name character varying(255) DEFAULT NULL::character varying,
    task_type character varying(50) DEFAULT NULL::character varying,
    task_code bigint NOT NULL,
    task_definition_version integer,
    process_instance_id integer,
    state integer,
    submit_time timestamp without time zone,
    start_time timestamp without time zone,
    end_time timestamp without time zone,
    host character varying(135) DEFAULT NULL::character varying,
    execute_path character varying(200) DEFAULT NULL::character varying,
    log_path text,
    alert_flag integer,
    retry_times integer DEFAULT 0,
    pid integer,
    app_link text,
    task_params text,
    flag integer DEFAULT 1,
    retry_interval integer,
    max_retry_times integer,
    task_instance_priority integer,
    worker_group character varying(64),
    environment_code bigint DEFAULT '-1'::bigint,
    environment_config text,
    executor_id integer,
    first_submit_time timestamp without time zone,
    delay_time integer DEFAULT 0,
    task_group_id integer,
    var_pool text,
    dry_run integer DEFAULT 0
);


ALTER TABLE public.t_ds_task_instance OWNER TO root;

--
-- Name: t_ds_tenant_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_tenant_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_tenant_id_sequence OWNER TO root;

--
-- Name: t_ds_tenant; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_tenant (
    id integer DEFAULT nextval('public.t_ds_tenant_id_sequence'::regclass) NOT NULL,
    tenant_code character varying(64) DEFAULT NULL::character varying,
    description character varying(255) DEFAULT NULL::character varying,
    queue_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_tenant OWNER TO root;

--
-- Name: t_ds_udfs_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_udfs_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_udfs_id_sequence OWNER TO root;

--
-- Name: t_ds_udfs; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_udfs (
    id integer DEFAULT nextval('public.t_ds_udfs_id_sequence'::regclass) NOT NULL,
    user_id integer NOT NULL,
    func_name character varying(100) NOT NULL,
    class_name character varying(255) NOT NULL,
    type integer NOT NULL,
    arg_types character varying(255) DEFAULT NULL::character varying,
    database character varying(255) DEFAULT NULL::character varying,
    description character varying(255) DEFAULT NULL::character varying,
    resource_id integer NOT NULL,
    resource_name character varying(255) NOT NULL,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone NOT NULL
);


ALTER TABLE public.t_ds_udfs OWNER TO root;

--
-- Name: t_ds_user_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_user_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_user_id_sequence OWNER TO root;

--
-- Name: t_ds_user; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_user (
    id integer DEFAULT nextval('public.t_ds_user_id_sequence'::regclass) NOT NULL,
    user_name character varying(64) DEFAULT NULL::character varying,
    user_password character varying(64) DEFAULT NULL::character varying,
    user_type integer,
    email character varying(64) DEFAULT NULL::character varying,
    phone character varying(11) DEFAULT NULL::character varying,
    tenant_id integer,
    create_time timestamp without time zone,
    update_time timestamp without time zone,
    queue character varying(64) DEFAULT NULL::character varying,
    state integer DEFAULT 1,
    time_zone character varying(32) DEFAULT NULL::character varying
);


ALTER TABLE public.t_ds_user OWNER TO root;

--
-- Name: COLUMN t_ds_user.state; Type: COMMENT; Schema: public; Owner: root
--

COMMENT ON COLUMN public.t_ds_user.state IS 'state 0:disable 1:enable';


--
-- Name: t_ds_version_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_version_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_version_id_sequence OWNER TO root;

--
-- Name: t_ds_version; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_version (
    id integer DEFAULT nextval('public.t_ds_version_id_sequence'::regclass) NOT NULL,
    version character varying(200) NOT NULL
);


ALTER TABLE public.t_ds_version OWNER TO root;

--
-- Name: t_ds_worker_group_id_sequence; Type: SEQUENCE; Schema: public; Owner: root
--

CREATE SEQUENCE public.t_ds_worker_group_id_sequence
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ds_worker_group_id_sequence OWNER TO root;

--
-- Name: t_ds_worker_group; Type: TABLE; Schema: public; Owner: root
--

CREATE TABLE public.t_ds_worker_group (
    id bigint DEFAULT nextval('public.t_ds_worker_group_id_sequence'::regclass) NOT NULL,
    name character varying(255) NOT NULL,
    addr_list text,
    create_time timestamp without time zone,
    update_time timestamp without time zone
);


ALTER TABLE public.t_ds_worker_group OWNER TO root;

--
-- Name: t_ds_alert_plugin_instance id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alert_plugin_instance ALTER COLUMN id SET DEFAULT nextval('public.t_ds_alert_plugin_instance_id_seq'::regclass);


--
-- Name: t_ds_alert_send_status id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alert_send_status ALTER COLUMN id SET DEFAULT nextval('public.t_ds_alert_send_status_id_seq'::regclass);


--
-- Name: t_ds_audit_log id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_audit_log ALTER COLUMN id SET DEFAULT nextval('public.t_ds_audit_log_id_seq'::regclass);


--
-- Name: t_ds_dq_comparison_type id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_comparison_type ALTER COLUMN id SET DEFAULT nextval('public.t_ds_dq_comparison_type_id_seq'::regclass);


--
-- Name: t_ds_dq_execute_result id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_execute_result ALTER COLUMN id SET DEFAULT nextval('public.t_ds_dq_execute_result_id_seq'::regclass);


--
-- Name: t_ds_dq_rule id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_rule ALTER COLUMN id SET DEFAULT nextval('public.t_ds_dq_rule_id_seq'::regclass);


--
-- Name: t_ds_dq_rule_execute_sql id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_rule_execute_sql ALTER COLUMN id SET DEFAULT nextval('public.t_ds_dq_rule_execute_sql_id_seq'::regclass);


--
-- Name: t_ds_dq_rule_input_entry id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_rule_input_entry ALTER COLUMN id SET DEFAULT nextval('public.t_ds_dq_rule_input_entry_id_seq'::regclass);


--
-- Name: t_ds_dq_task_statistics_value id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_task_statistics_value ALTER COLUMN id SET DEFAULT nextval('public.t_ds_dq_task_statistics_value_id_seq'::regclass);


--
-- Name: t_ds_environment id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment ALTER COLUMN id SET DEFAULT nextval('public.t_ds_environment_id_seq'::regclass);


--
-- Name: t_ds_environment_worker_group_relation id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment_worker_group_relation ALTER COLUMN id SET DEFAULT nextval('public.t_ds_environment_worker_group_relation_id_seq'::regclass);


--
-- Name: t_ds_k8s id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_k8s ALTER COLUMN id SET DEFAULT nextval('public.t_ds_k8s_id_seq'::regclass);


--
-- Name: t_ds_k8s_namespace id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_k8s_namespace ALTER COLUMN id SET DEFAULT nextval('public.t_ds_k8s_namespace_id_seq'::regclass);


--
-- Name: t_ds_plugin_define id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_plugin_define ALTER COLUMN id SET DEFAULT nextval('public.t_ds_plugin_define_id_seq'::regclass);


--
-- Name: t_ds_relation_namespace_user id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_namespace_user ALTER COLUMN id SET DEFAULT nextval('public.t_ds_relation_namespace_user_id_seq'::regclass);


--
-- Name: t_ds_relation_rule_execute_sql id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_rule_execute_sql ALTER COLUMN id SET DEFAULT nextval('public.t_ds_relation_rule_execute_sql_id_seq'::regclass);


--
-- Name: t_ds_relation_rule_input_entry id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_rule_input_entry ALTER COLUMN id SET DEFAULT nextval('public.t_ds_relation_rule_input_entry_id_seq'::regclass);


--
-- Name: t_ds_task_group id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_group ALTER COLUMN id SET DEFAULT nextval('public.t_ds_task_group_id_seq'::regclass);


--
-- Name: t_ds_task_group_queue id; Type: DEFAULT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_group_queue ALTER COLUMN id SET DEFAULT nextval('public.t_ds_task_group_queue_id_seq'::regclass);


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

INSERT INTO public.qrtz_locks VALUES ('DolphinScheduler', 'STATE_ACCESS');
INSERT INTO public.qrtz_locks VALUES ('DolphinScheduler', 'TRIGGER_ACCESS');


--
-- Data for Name: qrtz_paused_trigger_grps; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: qrtz_scheduler_state; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.qrtz_scheduler_state VALUES ('DolphinScheduler', 'VM-8-4-ubuntu1688030144147', 1688030165976, 5000);


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

INSERT INTO public.t_ds_access_token VALUES (1, 111, '111', '2023-08-31 16:36:05', '2023-08-31 16:36:11', '2023-08-31 16:36:15');
INSERT INTO public.t_ds_access_token VALUES (2, 22, '111', '2023-08-31 16:36:35', '2023-08-31 16:36:38', '2023-08-31 16:36:42');
INSERT INTO public.t_ds_access_token VALUES (3, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (4, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (5, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (6, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (7, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (8, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (9, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (10, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (11, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (12, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (13, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');
INSERT INTO public.t_ds_access_token VALUES (14, 22, '11', '2023-08-31 16:36:55', '2023-08-31 16:36:57', '2023-08-31 16:36:59');


--
-- Data for Name: t_ds_alert; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_alert VALUES (1, 'Fault tolerance warning', 'f5bc9bc42f1039d3bb7a079007620b36d9499bd1', '[{"type":"MASTER","host":"/nodes/master/10.0.8.4:5678","event":"SERVER_DOWN","warningLevel":"SERIOUS"}]', 0, 2, NULL, 1, '2023-06-29 16:27:41.297', '2023-06-29 16:27:41.297', NULL, NULL, NULL, 4);


--
-- Data for Name: t_ds_alert_plugin_instance; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: t_ds_alert_send_status; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: t_ds_alertgroup; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_alertgroup VALUES (1, '1,2', 1, 'default admin warning group', 'default admin warning group', '2018-11-29 10:20:39', '2018-11-29 10:20:39');


--
-- Data for Name: t_ds_audit_log; Type: TABLE DATA; Schema: public; Owner: root
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

INSERT INTO public.t_ds_dq_comparison_type VALUES (1, 'FixValue', NULL, NULL, NULL, '2021-06-30 00:00:00', '2021-06-30 00:00:00', false);
INSERT INTO public.t_ds_dq_comparison_type VALUES (2, 'DailyAvg', 'select round(avg(statistics_value),2) as day_avg from t_ds_dq_task_statistics_value where data_time >=date_trunc(''DAY'', ${data_time}) and data_time < date_add(date_trunc(''day'', ${data_time}),1) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''', 'day_range', 'day_range.day_avg', '2021-06-30 00:00:00', '2021-06-30 00:00:00', true);
INSERT INTO public.t_ds_dq_comparison_type VALUES (3, 'WeeklyAvg', 'select round(avg(statistics_value),2) as week_avg from t_ds_dq_task_statistics_value where  data_time >= date_trunc(''WEEK'', ${data_time}) and data_time <date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''', 'week_range', 'week_range.week_avg', '2021-06-30 00:00:00', '2021-06-30 00:00:00', true);
INSERT INTO public.t_ds_dq_comparison_type VALUES (4, 'MonthlyAvg', 'select round(avg(statistics_value),2) as month_avg from t_ds_dq_task_statistics_value where  data_time >= date_trunc(''MONTH'', ${data_time}) and data_time <date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''', 'month_range', 'month_range.month_avg', '2021-06-30 00:00:00', '2021-06-30 00:00:00', true);
INSERT INTO public.t_ds_dq_comparison_type VALUES (5, 'Last7DayAvg', 'select round(avg(statistics_value),2) as last_7_avg from t_ds_dq_task_statistics_value where  data_time >= date_add(date_trunc(''day'', ${data_time}),-7) and  data_time <date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''', 'last_seven_days', 'last_seven_days.last_7_avg', '2021-06-30 00:00:00', '2021-06-30 00:00:00', true);
INSERT INTO public.t_ds_dq_comparison_type VALUES (6, 'Last30DayAvg', 'select round(avg(statistics_value),2) as last_30_avg from t_ds_dq_task_statistics_value where  data_time >= date_add(date_trunc(''day'', ${data_time}),-30) and  data_time < date_trunc(''day'', ${data_time}) and unique_code = ${unique_code} and statistics_name = ''${statistics_name}''', 'last_thirty_days', 'last_thirty_days.last_30_avg', '2021-06-30 00:00:00', '2021-06-30 00:00:00', true);
INSERT INTO public.t_ds_dq_comparison_type VALUES (7, 'SrcTableTotalRows', 'SELECT COUNT(*) AS total FROM ${src_table} WHERE (${src_filter})', 'total_count', 'total_count.total', '2021-06-30 00:00:00', '2021-06-30 00:00:00', false);
INSERT INTO public.t_ds_dq_comparison_type VALUES (8, 'TargetTableTotalRows', 'SELECT COUNT(*) AS total FROM ${target_table} WHERE (${target_filter})', 'total_count', 'total_count.total', '2021-06-30 00:00:00', '2021-06-30 00:00:00', false);


--
-- Data for Name: t_ds_dq_execute_result; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: t_ds_dq_rule; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_dq_rule VALUES (1, '$t(null_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (2, '$t(custom_sql)', 1, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (3, '$t(multi_table_accuracy)', 2, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (4, '$t(multi_table_value_comparison)', 3, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (5, '$t(field_length_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (6, '$t(uniqueness_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (7, '$t(regexp_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (8, '$t(timeliness_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (9, '$t(enumeration_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');
INSERT INTO public.t_ds_dq_rule VALUES (10, '$t(table_count_check)', 0, 1, '2020-01-12 00:00:00', '2020-01-12 00:00:00');


--
-- Data for Name: t_ds_dq_rule_execute_sql; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (1, 1, 'SELECT COUNT(*) AS nulls FROM null_items', 'null_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (2, 1, 'SELECT COUNT(*) AS total FROM ${src_table} WHERE (${src_filter})', 'total_count', 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (3, 1, 'SELECT COUNT(*) AS miss from miss_items', 'miss_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (4, 1, 'SELECT COUNT(*) AS valids FROM invalid_length_items', 'invalid_length_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (5, 1, 'SELECT COUNT(*) AS total FROM ${target_table} WHERE (${target_filter})', 'total_count', 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (6, 1, 'SELECT ${src_field} FROM ${src_table} group by ${src_field} having count(*) > 1', 'duplicate_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (7, 1, 'SELECT COUNT(*) AS duplicates FROM duplicate_items', 'duplicate_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (8, 1, 'SELECT ${src_table}.* FROM (SELECT * FROM ${src_table} WHERE (${src_filter})) ${src_table} LEFT JOIN (SELECT * FROM ${target_table} WHERE (${target_filter})) ${target_table} ON ${on_clause} WHERE ${where_clause}', 'miss_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (9, 1, 'SELECT * FROM ${src_table} WHERE (${src_field} not regexp ''${regexp_pattern}'') AND (${src_filter}) ', 'regexp_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (10, 1, 'SELECT COUNT(*) AS regexps FROM regexp_items', 'regexp_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (11, 1, 'SELECT * FROM ${src_table} WHERE (to_unix_timestamp(${src_field}, ''${datetime_format}'')-to_unix_timestamp(''${deadline}'', ''${datetime_format}'') <= 0) AND (to_unix_timestamp(${src_field}, ''${datetime_format}'')-to_unix_timestamp(''${begin_time}'', ''${datetime_format}'') >= 0) AND (${src_filter}) ', 'timeliness_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (12, 1, 'SELECT COUNT(*) AS timeliness FROM timeliness_items', 'timeliness_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (13, 1, 'SELECT * FROM ${src_table} where (${src_field} not in ( ${enum_list} ) or ${src_field} is null) AND (${src_filter}) ', 'enum_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (14, 1, 'SELECT COUNT(*) AS enums FROM enum_items', 'enum_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (15, 1, 'SELECT COUNT(*) AS total FROM ${src_table} WHERE (${src_filter})', 'table_count', 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24', false);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (16, 1, 'SELECT * FROM ${src_table} WHERE (${src_field} is null or ${src_field} = '''') AND (${src_filter})', 'null_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);
INSERT INTO public.t_ds_dq_rule_execute_sql VALUES (17, 1, 'SELECT * FROM ${src_table} WHERE (length(${src_field}) ${logic_operator} ${field_length}) AND (${src_filter})', 'invalid_length_items', 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24', true);


--
-- Data for Name: t_ds_dq_rule_input_entry; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_dq_rule_input_entry VALUES (1, 'src_connector_type', 'select', '$t(src_connector_type)', '', '[{"label":"HIVE","value":"HIVE"},{"label":"JDBC","value":"JDBC"}]', 'please select source connector type', 2, 2, 0, 1, 1, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (2, 'src_datasource_id', 'select', '$t(src_datasource_id)', '', NULL, 'please select source datasource id', 1, 2, 0, 1, 1, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (3, 'src_table', 'select', '$t(src_table)', NULL, NULL, 'Please enter source table name', 0, 0, 0, 1, 1, 1, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (4, 'src_filter', 'input', '$t(src_filter)', NULL, NULL, 'Please enter filter expression', 0, 3, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (5, 'src_field', 'select', '$t(src_field)', NULL, NULL, 'Please enter column, only single column is supported', 0, 0, 0, 1, 1, 0, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (6, 'statistics_name', 'input', '$t(statistics_name)', NULL, NULL, 'Please enter statistics name, the alias in statistics execute sql', 0, 0, 1, 0, 0, 0, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (7, 'check_type', 'select', '$t(check_type)', '0', '[{"label":"Expected - Actual","value":"0"},{"label":"Actual - Expected","value":"1"},{"label":"Actual / Expected","value":"2"},{"label":"(Expected - Actual) / Expected","value":"3"}]', 'please select check type', 0, 0, 3, 1, 1, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (8, 'operator', 'select', '$t(operator)', '0', '[{"label":"=","value":"0"},{"label":"<","value":"1"},{"label":"<=","value":"2"},{"label":">","value":"3"},{"label":">=","value":"4"},{"label":"!=","value":"5"}]', 'please select operator', 0, 0, 3, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (9, 'threshold', 'input', '$t(threshold)', NULL, NULL, 'Please enter threshold, number is needed', 0, 2, 3, 1, 1, 0, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (10, 'failure_strategy', 'select', '$t(failure_strategy)', '0', '[{"label":"Alert","value":"0"},{"label":"Block","value":"1"}]', 'please select failure strategy', 0, 0, 3, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (11, 'target_connector_type', 'select', '$t(target_connector_type)', '', '[{"label":"HIVE","value":"HIVE"},{"label":"JDBC","value":"JDBC"}]', 'Please select target connector type', 2, 0, 0, 1, 1, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (12, 'target_datasource_id', 'select', '$t(target_datasource_id)', '', NULL, 'Please select target datasource', 1, 2, 0, 1, 1, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (13, 'target_table', 'select', '$t(target_table)', NULL, NULL, 'Please enter target table', 0, 0, 0, 1, 1, 1, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (14, 'target_filter', 'input', '$t(target_filter)', NULL, NULL, 'Please enter target filter expression', 0, 3, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (15, 'mapping_columns', 'group', '$t(mapping_columns)', NULL, '[{"field":"src_field","props":{"placeholder":"Please input src field","rows":0,"disabled":false,"size":"small"},"type":"input","title":"src_field"},{"field":"operator","props":{"placeholder":"Please input operator","rows":0,"disabled":false,"size":"small"},"type":"input","title":"operator"},{"field":"target_field","props":{"placeholder":"Please input target field","rows":0,"disabled":false,"size":"small"},"type":"input","title":"target_field"}]', 'please enter mapping columns', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (16, 'statistics_execute_sql', 'textarea', '$t(statistics_execute_sql)', NULL, NULL, 'Please enter statistics execute sql', 0, 3, 0, 1, 1, 0, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (17, 'comparison_name', 'input', '$t(comparison_name)', NULL, NULL, 'Please enter comparison name, the alias in comparison execute sql', 0, 0, 0, 0, 0, 0, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (18, 'comparison_execute_sql', 'textarea', '$t(comparison_execute_sql)', NULL, NULL, 'Please enter comparison execute sql', 0, 3, 0, 1, 1, 0, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (19, 'comparison_type', 'select', '$t(comparison_type)', '', NULL, 'Please enter comparison title', 3, 0, 2, 1, 0, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (20, 'writer_connector_type', 'select', '$t(writer_connector_type)', '', '[{"label":"MYSQL","value":"0"},{"label":"POSTGRESQL","value":"1"}]', 'please select writer connector type', 0, 2, 0, 1, 1, 1, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (21, 'writer_datasource_id', 'select', '$t(writer_datasource_id)', '', NULL, 'please select writer datasource id', 1, 2, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (22, 'target_field', 'select', '$t(target_field)', NULL, NULL, 'Please enter column, only single column is supported', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (23, 'field_length', 'input', '$t(field_length)', NULL, NULL, 'Please enter length limit', 0, 3, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (24, 'logic_operator', 'select', '$t(logic_operator)', '=', '[{"label":"=","value":"="},{"label":"<","value":"<"},{"label":"<=","value":"<="},{"label":">","value":">"},{"label":">=","value":">="},{"label":"<>","value":"<>"}]', 'please select logic operator', 0, 0, 3, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (25, 'regexp_pattern', 'input', '$t(regexp_pattern)', NULL, NULL, 'Please enter regexp pattern', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (26, 'deadline', 'input', '$t(deadline)', NULL, NULL, 'Please enter deadline', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (27, 'datetime_format', 'input', '$t(datetime_format)', NULL, NULL, 'Please enter datetime format', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (28, 'enum_list', 'input', '$t(enum_list)', NULL, NULL, 'Please enter enumeration', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_dq_rule_input_entry VALUES (29, 'begin_time', 'input', '$t(begin_time)', NULL, NULL, 'Please enter begin time', 0, 0, 0, 1, 1, 0, 0, '2021-03-03 11:31:24', '2021-03-03 11:31:24');


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
-- Data for Name: t_ds_k8s; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: t_ds_k8s_namespace; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: t_ds_plugin_define; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_plugin_define VALUES (1, 'DingTalk', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":null,"field":"WebHook","name":"$t(''webhook'')","type":"input","title":"$t(''webhook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Keyword","name":"$t(''keyword'')","type":"input","title":"$t(''keyword'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Secret","name":"$t(''secret'')","type":"input","title":"$t(''secret'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"MsgType","name":"$t(''msgType'')","type":"radio","title":"$t(''msgType'')","value":"text","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"text","value":"text","disabled":false},{"label":"markdown","value":"markdown","disabled":false}]},{"props":null,"field":"AtMobiles","name":"$t(''atMobiles'')","type":"input","title":"$t(''atMobiles'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"AtUserIds","name":"$t(''atUserIds'')","type":"input","title":"$t(''atUserIds'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"IsAtAll","name":"$t(''isAtAll'')","type":"radio","title":"$t(''isAtAll'')","value":"false","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"IsEnableProxy","name":"$t(''isEnableProxy'')","type":"radio","title":"$t(''isEnableProxy'')","value":"false","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"Proxy","name":"$t(''proxy'')","type":"input","title":"$t(''proxy'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Port","name":"$t(''port'')","type":"input","title":"$t(''port'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"User","name":"$t(''user'')","type":"input","title":"$t(''user'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"field":"Password","name":"$t(''password'')","props":{"disabled":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"type":"input","title":"$t(''password'')","value":null,"validate":null,"emit":null}]', '2023-06-29 15:48:19.191', '2023-06-29 15:48:19.191');
INSERT INTO public.t_ds_plugin_define VALUES (2, 'Email', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input receives","size":"small"},"field":"receivers","name":"$t(''receivers'')","type":"input","title":"$t(''receivers'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"receiverCcs","name":"$t(''receiverCcs'')","type":"input","title":"$t(''receiverCcs'')","value":null,"validate":null,"emit":null},{"props":null,"field":"serverHost","name":"mail.smtp.host","type":"input","title":"mail.smtp.host","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"serverPort","name":"mail.smtp.port","type":"input","title":"mail.smtp.port","value":"25","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"sender","name":"$t(''mailSender'')","type":"input","title":"$t(''mailSender'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"enableSmtpAuth","name":"mail.smtp.auth","type":"radio","title":"mail.smtp.auth","value":"true","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"if enable use authentication, you need input user","size":"small"},"field":"User","name":"$t(''mailUser'')","type":"input","title":"$t(''mailUser'')","value":null,"validate":null,"emit":null},{"field":"Password","name":"$t(''mailPasswd'')","props":{"disabled":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"type":"input","title":"$t(''mailPasswd'')","value":null,"validate":null,"emit":null},{"props":null,"field":"starttlsEnable","name":"mail.smtp.starttls.enable","type":"radio","title":"mail.smtp.starttls.enable","value":"false","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"sslEnable","name":"mail.smtp.ssl.enable","type":"radio","title":"mail.smtp.ssl.enable","value":"false","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"smtpSslTrust","name":"mail.smtp.ssl.trust","type":"input","title":"mail.smtp.ssl.trust","value":"*","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"showType","name":"$t(''showType'')","type":"radio","title":"$t(''showType'')","value":"table","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"table","value":"table","disabled":false},{"label":"text","value":"text","disabled":false},{"label":"attachment","value":"attachment","disabled":false},{"label":"table attachment","value":"table attachment","disabled":false}]}]', '2023-06-29 15:48:19.283', '2023-06-29 15:48:19.283');
INSERT INTO public.t_ds_plugin_define VALUES (3, 'Feishu', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":null,"field":"WebHook","name":"$t(''webhook'')","type":"input","title":"$t(''webhook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"IsEnableProxy","name":"$t(''isEnableProxy'')","type":"radio","title":"$t(''isEnableProxy'')","value":"true","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"Proxy","name":"$t(''proxy'')","type":"input","title":"$t(''proxy'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Port","name":"$t(''port'')","type":"input","title":"$t(''port'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"User","name":"$t(''user'')","type":"input","title":"$t(''user'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"field":"Password","name":"$t(''password'')","props":{"disabled":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"type":"input","title":"$t(''password'')","value":null,"validate":null,"emit":null}]', '2023-06-29 15:48:19.339', '2023-06-29 15:48:19.339');
INSERT INTO public.t_ds_plugin_define VALUES (4, 'Http', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request URL","size":"small"},"field":"url","name":"$t(''url'')","type":"input","title":"$t(''url'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request type POST or GET","size":"small"},"field":"requestType","name":"$t(''requestType'')","type":"input","title":"$t(''requestType'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request headers as JSON format ","size":"small"},"field":"headerParams","name":"$t(''headerParams'')","type":"input","title":"$t(''headerParams'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input request body as JSON format ","size":"small"},"field":"bodyParams","name":"$t(''bodyParams'')","type":"input","title":"$t(''bodyParams'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"input alert msg field name","size":"small"},"field":"contentField","name":"$t(''contentField'')","type":"input","title":"$t(''contentField'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]', '2023-06-29 15:48:19.372', '2023-06-29 15:48:19.372');
INSERT INTO public.t_ds_plugin_define VALUES (5, 'Script', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please enter your custom parameters, which will be passed to you when calling your script","size":"small"},"field":"userParams","name":"$t(''userParams'')","type":"input","title":"$t(''userParams'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please upload the file to the disk directory of the alert server, and ensure that the path is absolute and has the corresponding access rights","size":"small"},"field":"path","name":"$t(''scriptPath'')","type":"input","title":"$t(''scriptPath'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"type","name":"$t(''scriptType'')","type":"radio","title":"$t(''scriptType'')","value":"SHELL","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"SHELL","value":"SHELL","disabled":false}]}]', '2023-06-29 15:48:19.397', '2023-06-29 15:48:19.397');
INSERT INTO public.t_ds_plugin_define VALUES (6, 'Slack', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"Input WebHook Url","size":"small"},"field":"webHook","name":"$t(''webhook'')","type":"input","title":"$t(''webhook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"Input the bot username","size":"small"},"field":"username","name":"$t(''Username'')","type":"input","title":"$t(''Username'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]', '2023-06-29 15:48:19.417', '2023-06-29 15:48:19.417');
INSERT INTO public.t_ds_plugin_define VALUES (7, 'WeChat', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input corp id ","size":"small"},"field":"corpId","name":"$t(''corpId'')","type":"input","title":"$t(''corpId'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input secret ","size":"small"},"field":"secret","name":"$t(''secret'')","type":"input","title":"$t(''secret'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"use `|` to separate userIds and `@all` to everyone ","size":"small"},"field":"users","name":"$t(''users'')","type":"input","title":"$t(''users'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"please input agent id or chat id ","size":"small"},"field":"agentId/chatId","name":"$t(''agentId/chatId'')","type":"input","title":"$t(''agentId/chatId'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"sendType","name":"send.type","type":"radio","title":"send.type","value":"APP/应用","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"APP/应用","value":"APP/应用","disabled":false},{"label":"GROUP CHAT/群聊","value":"GROUP CHAT/群聊","disabled":false}]},{"props":null,"field":"showType","name":"$t(''showType'')","type":"radio","title":"$t(''showType'')","value":"markdown","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"markdown","value":"markdown","disabled":false},{"label":"text","value":"text","disabled":false}]}]', '2023-06-29 15:48:19.442', '2023-06-29 15:48:19.442');
INSERT INTO public.t_ds_plugin_define VALUES (8, 'PagerDuty', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":null,"field":"IntegrationKey","name":"integrationKey","type":"input","title":"integrationKey","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]', '2023-06-29 15:48:19.464', '2023-06-29 15:48:19.465');
INSERT INTO public.t_ds_plugin_define VALUES (9, 'WebexTeams', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"Please enter the robot''s access token you were given","size":"small"},"field":"BotAccessToken","name":"botAccessToken","type":"input","title":"botAccessToken","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"The room ID of the message","size":"small"},"field":"RoomId","name":"roomId","type":"input","title":"roomId","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"The person ID of the message recipient","size":"small"},"field":"ToPersonId","name":"toPersonId","type":"input","title":"toPersonId","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"The email address of the message recipient","size":"small"},"field":"ToPersonEmail","name":"toPersonEmail","type":"input","title":"toPersonEmail","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"use ,(eng commas) to separate multiple emails","size":"small"},"field":"AtSomeoneInRoom","name":"atSomeoneInRoom","type":"input","title":"atSomeoneInRoom","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Destination","name":"destination","type":"radio","title":"destination","value":"roomId","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"roomId","value":"roomId","disabled":false},{"label":"personEmail","value":"personEmail","disabled":false},{"label":"personId","value":"personId","disabled":false}]}]', '2023-06-29 15:48:19.489', '2023-06-29 15:48:19.489');
INSERT INTO public.t_ds_plugin_define VALUES (10, 'Telegram', 'alert', '[{"props":null,"field":"WarningType","name":"warningType","type":"radio","title":"warningType","value":"all","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"success","value":"success","disabled":false},{"label":"failure","value":"failure","disabled":false},{"label":"all","value":"all","disabled":false}]},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"telegram web hook","size":"small"},"field":"webHook","name":"$t(''webHook'')","type":"input","title":"$t(''webHook'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"telegram bot token","size":"small"},"field":"botToken","name":"botToken","type":"input","title":"botToken","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":{"disabled":null,"type":null,"maxlength":null,"minlength":null,"clearable":null,"prefixIcon":null,"suffixIcon":null,"rows":null,"autosize":null,"autocomplete":null,"name":null,"readonly":null,"max":null,"min":null,"step":null,"resize":null,"autofocus":null,"form":null,"label":null,"tabindex":null,"validateEvent":null,"showPassword":null,"placeholder":"telegram channel chat id","size":"small"},"field":"chatId","name":"chatId","type":"input","title":"chatId","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"field":"parseMode","name":"parseMode","props":{"disabled":null,"placeholder":null,"size":"small"},"type":"select","title":"parseMode","value":"Txt","validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"Txt","value":"Txt","disabled":false},{"label":"Markdown","value":"Markdown","disabled":false},{"label":"MarkdownV2","value":"MarkdownV2","disabled":false},{"label":"Html","value":"Html","disabled":false}]},{"props":null,"field":"IsEnableProxy","name":"$t(''isEnableProxy'')","type":"radio","title":"$t(''isEnableProxy'')","value":"false","validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null,"options":[{"label":"YES","value":"true","disabled":false},{"label":"NO","value":"false","disabled":false}]},{"props":null,"field":"Proxy","name":"$t(''proxy'')","type":"input","title":"$t(''proxy'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"Port","name":"$t(''port'')","type":"input","title":"$t(''port'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"User","name":"$t(''user'')","type":"input","title":"$t(''user'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"field":"Password","name":"$t(''password'')","props":{"disabled":null,"placeholder":"if enable use authentication, you need input password","size":"small"},"type":"input","title":"$t(''password'')","value":null,"validate":[{"required":false,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]', '2023-06-29 15:48:19.771', '2023-06-29 15:48:19.771');
INSERT INTO public.t_ds_plugin_define VALUES (11, 'CONDITIONS', 'task', '[{"props":null,"field":"name","name":"$t(''Node name'')","type":"input","title":"$t(''Node name'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"runFlag","name":"RUN_FLAG","type":"radio","title":"RUN_FLAG","value":null,"validate":null,"emit":null,"options":[{"label":"NORMAL","value":"NORMAL","disabled":false},{"label":"FORBIDDEN","value":"FORBIDDEN","disabled":false}]}]', '2023-06-29 16:15:03.606', '2023-06-29 16:15:03.606');
INSERT INTO public.t_ds_plugin_define VALUES (12, 'DATA_QUALITY', 'task', 'null', '2023-06-29 16:15:03.668', '2023-06-29 16:15:03.668');
INSERT INTO public.t_ds_plugin_define VALUES (13, 'DATAX', 'task', 'null', '2023-06-29 16:15:03.687', '2023-06-29 16:15:03.687');
INSERT INTO public.t_ds_plugin_define VALUES (14, 'DEPENDENT', 'task', 'null', '2023-06-29 16:15:03.697', '2023-06-29 16:15:03.697');
INSERT INTO public.t_ds_plugin_define VALUES (15, 'FLINK', 'task', 'null', '2023-06-29 16:15:03.709', '2023-06-29 16:15:03.709');
INSERT INTO public.t_ds_plugin_define VALUES (16, 'HTTP', 'task', 'null', '2023-06-29 16:15:03.721', '2023-06-29 16:15:03.721');
INSERT INTO public.t_ds_plugin_define VALUES (17, 'MR', 'task', 'null', '2023-06-29 16:15:03.745', '2023-06-29 16:15:03.745');
INSERT INTO public.t_ds_plugin_define VALUES (18, 'PIGEON', 'task', '[{"props":null,"field":"targetJobName","name":"targetJobName","type":"input","title":"targetJobName","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null}]', '2023-06-29 16:15:03.765', '2023-06-29 16:15:03.765');
INSERT INTO public.t_ds_plugin_define VALUES (19, 'PROCEDURE', 'task', 'null', '2023-06-29 16:15:03.781', '2023-06-29 16:15:03.781');
INSERT INTO public.t_ds_plugin_define VALUES (20, 'PYTHON', 'task', 'null', '2023-06-29 16:15:03.82', '2023-06-29 16:15:03.82');
INSERT INTO public.t_ds_plugin_define VALUES (21, 'SEATUNNEL', 'task', 'null', '2023-06-29 16:15:03.838', '2023-06-29 16:15:03.838');
INSERT INTO public.t_ds_plugin_define VALUES (22, 'SHELL', 'task', '[{"props":null,"field":"name","name":"$t(''Node name'')","type":"input","title":"$t(''Node name'')","value":null,"validate":[{"required":true,"message":null,"type":"string","trigger":"blur","min":null,"max":null}],"emit":null},{"props":null,"field":"runFlag","name":"RUN_FLAG","type":"radio","title":"RUN_FLAG","value":null,"validate":null,"emit":null,"options":[{"label":"NORMAL","value":"NORMAL","disabled":false},{"label":"FORBIDDEN","value":"FORBIDDEN","disabled":false}]}]', '2023-06-29 16:15:03.864', '2023-06-29 16:15:03.864');
INSERT INTO public.t_ds_plugin_define VALUES (23, 'SPARK', 'task', 'null', '2023-06-29 16:15:03.881', '2023-06-29 16:15:03.881');
INSERT INTO public.t_ds_plugin_define VALUES (24, 'SQL', 'task', 'null', '2023-06-29 16:15:03.898', '2023-06-29 16:15:03.898');
INSERT INTO public.t_ds_plugin_define VALUES (25, 'SQOOP', 'task', 'null', '2023-06-29 16:15:03.911', '2023-06-29 16:15:03.911');
INSERT INTO public.t_ds_plugin_define VALUES (26, 'SUB_PROCESS', 'task', 'null', '2023-06-29 16:15:03.926', '2023-06-29 16:15:03.926');
INSERT INTO public.t_ds_plugin_define VALUES (27, 'SWITCH', 'task', 'null', '2023-06-29 16:15:03.947', '2023-06-29 16:15:03.947');
INSERT INTO public.t_ds_plugin_define VALUES (28, 'EMR', 'task', '[]', '2023-06-29 16:15:03.964', '2023-06-29 16:15:03.964');
INSERT INTO public.t_ds_plugin_define VALUES (29, 'ZEPPELIN', 'task', 'null', '2023-06-29 16:15:03.979', '2023-06-29 16:15:03.979');
INSERT INTO public.t_ds_plugin_define VALUES (30, 'BLOCKING', 'task', 'null', '2023-06-29 16:15:03.994', '2023-06-29 16:15:03.994');


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



--
-- Data for Name: t_ds_queue; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_queue VALUES (1, 'default', 'default', '2018-11-29 10:22:33', '2018-11-29 10:22:33');


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

INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (1, 1, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (3, 5, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (2, 3, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (4, 3, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (5, 6, 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (6, 6, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (7, 7, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (8, 7, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (9, 8, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (10, 8, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (11, 9, 13, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (12, 9, 14, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (13, 10, 15, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (14, 1, 16, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_execute_sql VALUES (15, 5, 17, '2021-03-03 11:31:24', '2021-03-03 11:31:24');


--
-- Data for Name: t_ds_relation_rule_input_entry; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_relation_rule_input_entry VALUES (1, 1, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (2, 1, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (3, 1, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (4, 1, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (5, 1, 5, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (6, 1, 6, '{"statistics_name":"null_count.nulls"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (7, 1, 7, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (8, 1, 8, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (9, 1, 9, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (10, 1, 10, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (11, 1, 17, '', 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (12, 1, 19, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (13, 2, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (14, 2, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (15, 2, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (16, 2, 6, '{"is_show":"true","can_edit":"true"}', 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (17, 2, 16, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (18, 2, 4, NULL, 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (19, 2, 7, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (20, 2, 8, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (21, 2, 9, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (22, 2, 10, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (24, 2, 19, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (25, 3, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (26, 3, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (27, 3, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (28, 3, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (29, 3, 11, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (30, 3, 12, NULL, 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (31, 3, 13, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (32, 3, 14, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (33, 3, 15, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (34, 3, 7, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (35, 3, 8, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (36, 3, 9, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (37, 3, 10, NULL, 13, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (38, 3, 17, '{"comparison_name":"total_count.total"}', 14, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (39, 3, 19, NULL, 15, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (40, 4, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (41, 4, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (42, 4, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (43, 4, 6, '{"is_show":"true","can_edit":"true"}', 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (44, 4, 16, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (45, 4, 11, NULL, 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (46, 4, 12, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (47, 4, 13, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (48, 4, 17, '{"is_show":"true","can_edit":"true"}', 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (49, 4, 18, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (50, 4, 7, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (51, 4, 8, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (52, 4, 9, NULL, 13, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (53, 4, 10, NULL, 14, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (62, 3, 6, '{"statistics_name":"miss_count.miss"}', 18, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (63, 5, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (64, 5, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (65, 5, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (66, 5, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (67, 5, 5, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (68, 5, 6, '{"statistics_name":"invalid_length_count.valids"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (69, 5, 24, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (70, 5, 23, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (71, 5, 7, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (72, 5, 8, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (73, 5, 9, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (74, 5, 10, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (75, 5, 17, '', 13, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (76, 5, 19, NULL, 14, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (79, 6, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (80, 6, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (81, 6, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (82, 6, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (83, 6, 5, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (84, 6, 6, '{"statistics_name":"duplicate_count.duplicates"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (85, 6, 7, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (86, 6, 8, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (87, 6, 9, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (88, 6, 10, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (89, 6, 17, '', 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (90, 6, 19, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (93, 7, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (94, 7, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (95, 7, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (96, 7, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (97, 7, 5, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (98, 7, 6, '{"statistics_name":"regexp_count.regexps"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (99, 7, 25, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (100, 7, 7, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (101, 7, 8, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (102, 7, 9, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (103, 7, 10, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (104, 7, 17, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (105, 7, 19, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (108, 8, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (109, 8, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (110, 8, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (111, 8, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (112, 8, 5, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (113, 8, 6, '{"statistics_name":"timeliness_count.timeliness"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (114, 8, 26, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (115, 8, 27, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (116, 8, 7, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (117, 8, 8, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (118, 8, 9, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (119, 8, 10, NULL, 13, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (120, 8, 17, NULL, 14, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (121, 8, 19, NULL, 15, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (124, 9, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (125, 9, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (126, 9, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (127, 9, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (128, 9, 5, NULL, 5, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (129, 9, 6, '{"statistics_name":"enum_count.enums"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (130, 9, 28, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (131, 9, 7, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (132, 9, 8, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (133, 9, 9, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (134, 9, 10, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (135, 9, 17, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (136, 9, 19, NULL, 13, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (139, 10, 1, NULL, 1, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (140, 10, 2, NULL, 2, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (141, 10, 3, NULL, 3, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (142, 10, 4, NULL, 4, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (143, 10, 6, '{"statistics_name":"table_count.total"}', 6, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (144, 10, 7, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (145, 10, 8, NULL, 8, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (146, 10, 9, NULL, 9, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (147, 10, 10, NULL, 10, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (148, 10, 17, NULL, 11, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (149, 10, 19, NULL, 12, '2021-03-03 11:31:24', '2021-03-03 11:31:24');
INSERT INTO public.t_ds_relation_rule_input_entry VALUES (150, 8, 29, NULL, 7, '2021-03-03 11:31:24', '2021-03-03 11:31:24');


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

INSERT INTO public.t_ds_session VALUES ('4757b696-676f-461e-bee7-87d2d864e3e2', 1, '127.0.0.1', '2023-10-11 16:50:45.275841');


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



--
-- Data for Name: t_ds_udfs; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Data for Name: t_ds_user; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_user VALUES (1, 'admin', '7ad2410b2f4c074479a8937a28a22b8f', 0, 'xxx@qq.com', '', 0, '2018-03-27 15:48:50', '2018-10-24 17:40:22', NULL, 1, NULL);


--
-- Data for Name: t_ds_version; Type: TABLE DATA; Schema: public; Owner: root
--

INSERT INTO public.t_ds_version VALUES (1, '3.0.1');


--
-- Data for Name: t_ds_worker_group; Type: TABLE DATA; Schema: public; Owner: root
--



--
-- Name: t_ds_access_token_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_access_token_id_sequence', 1, true);


--
-- Name: t_ds_alert_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_alert_id_sequence', 1, true);


--
-- Name: t_ds_alert_plugin_instance_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_alert_plugin_instance_id_seq', 1, false);


--
-- Name: t_ds_alert_send_status_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_alert_send_status_id_seq', 1, false);


--
-- Name: t_ds_alertgroup_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_alertgroup_id_sequence', 1, true);


--
-- Name: t_ds_audit_log_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_audit_log_id_seq', 1, false);


--
-- Name: t_ds_command_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_command_id_sequence', 1, false);


--
-- Name: t_ds_datasource_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_datasource_id_sequence', 1, false);


--
-- Name: t_ds_dq_comparison_type_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_dq_comparison_type_id_seq', 1, false);


--
-- Name: t_ds_dq_execute_result_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_dq_execute_result_id_seq', 1, false);


--
-- Name: t_ds_dq_rule_execute_sql_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_dq_rule_execute_sql_id_seq', 1, false);


--
-- Name: t_ds_dq_rule_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_dq_rule_id_seq', 1, false);


--
-- Name: t_ds_dq_rule_input_entry_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_dq_rule_input_entry_id_seq', 1, false);


--
-- Name: t_ds_dq_task_statistics_value_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_dq_task_statistics_value_id_seq', 1, false);


--
-- Name: t_ds_environment_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_environment_id_seq', 1, false);


--
-- Name: t_ds_environment_worker_group_relation_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_environment_worker_group_relation_id_seq', 1, false);


--
-- Name: t_ds_k8s_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_k8s_id_seq', 1, false);


--
-- Name: t_ds_k8s_namespace_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_k8s_namespace_id_seq', 1, false);


--
-- Name: t_ds_plugin_define_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_plugin_define_id_seq', 30, true);


--
-- Name: t_ds_process_definition_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_process_definition_id_sequence', 1, false);


--
-- Name: t_ds_process_definition_log_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_process_definition_log_id_sequence', 1, false);


--
-- Name: t_ds_process_instance_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_process_instance_id_sequence', 1, false);


--
-- Name: t_ds_process_task_relation_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_process_task_relation_id_sequence', 1, false);


--
-- Name: t_ds_process_task_relation_log_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_process_task_relation_log_id_sequence', 1, false);


--
-- Name: t_ds_project_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_project_id_sequence', 1, false);


--
-- Name: t_ds_queue_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_queue_id_sequence', 1, true);


--
-- Name: t_ds_relation_datasource_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_datasource_user_id_sequence', 1, false);


--
-- Name: t_ds_relation_namespace_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_namespace_user_id_seq', 1, false);


--
-- Name: t_ds_relation_process_instance_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_process_instance_id_sequence', 1, false);


--
-- Name: t_ds_relation_project_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_project_user_id_sequence', 1, false);


--
-- Name: t_ds_relation_resources_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_resources_user_id_sequence', 1, false);


--
-- Name: t_ds_relation_rule_execute_sql_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_rule_execute_sql_id_seq', 1, false);


--
-- Name: t_ds_relation_rule_input_entry_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_rule_input_entry_id_seq', 1, false);


--
-- Name: t_ds_relation_udfs_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_relation_udfs_user_id_sequence', 1, false);


--
-- Name: t_ds_resources_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_resources_id_sequence', 1, false);


--
-- Name: t_ds_schedules_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_schedules_id_sequence', 1, false);


--
-- Name: t_ds_task_definition_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_task_definition_id_sequence', 1, false);


--
-- Name: t_ds_task_definition_log_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_task_definition_log_id_sequence', 1, false);


--
-- Name: t_ds_task_group_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_task_group_id_seq', 1, false);


--
-- Name: t_ds_task_group_queue_id_seq; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_task_group_queue_id_seq', 1, false);


--
-- Name: t_ds_task_instance_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_task_instance_id_sequence', 1, false);


--
-- Name: t_ds_tenant_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_tenant_id_sequence', 1, false);


--
-- Name: t_ds_udfs_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_udfs_id_sequence', 1, false);


--
-- Name: t_ds_user_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_user_id_sequence', 1, true);


--
-- Name: t_ds_version_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_version_id_sequence', 1, true);


--
-- Name: t_ds_worker_group_id_sequence; Type: SEQUENCE SET; Schema: public; Owner: root
--

SELECT pg_catalog.setval('public.t_ds_worker_group_id_sequence', 1, false);


--
-- Name: t_ds_alert_send_status alert_send_status_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alert_send_status
    ADD CONSTRAINT alert_send_status_unique UNIQUE (alert_id, alert_plugin_instance_id);


--
-- Name: t_ds_environment environment_code_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment
    ADD CONSTRAINT environment_code_unique UNIQUE (code);


--
-- Name: t_ds_environment environment_name_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment
    ADD CONSTRAINT environment_name_unique UNIQUE (name);


--
-- Name: t_ds_environment_worker_group_relation environment_worker_group_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment_worker_group_relation
    ADD CONSTRAINT environment_worker_group_unique UNIQUE (environment_code, worker_group);


--
-- Name: t_ds_k8s_namespace k8s_namespace_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_k8s_namespace
    ADD CONSTRAINT k8s_namespace_unique UNIQUE (namespace, k8s);


--
-- Name: t_ds_worker_group name_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_worker_group
    ADD CONSTRAINT name_unique UNIQUE (name);


--
-- Name: t_ds_relation_namespace_user namespace_user_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_namespace_user
    ADD CONSTRAINT namespace_user_unique UNIQUE (user_id, namespace_id);


--
-- Name: t_ds_process_definition process_definition_unique; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_process_definition
    ADD CONSTRAINT process_definition_unique UNIQUE (name, project_code);


--
-- Name: qrtz_blob_triggers qrtz_blob_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_blob_triggers
    ADD CONSTRAINT qrtz_blob_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);


--
-- Name: qrtz_calendars qrtz_calendars_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_calendars
    ADD CONSTRAINT qrtz_calendars_pkey PRIMARY KEY (sched_name, calendar_name);


--
-- Name: qrtz_cron_triggers qrtz_cron_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_cron_triggers
    ADD CONSTRAINT qrtz_cron_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);


--
-- Name: qrtz_fired_triggers qrtz_fired_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_fired_triggers
    ADD CONSTRAINT qrtz_fired_triggers_pkey PRIMARY KEY (sched_name, entry_id);


--
-- Name: qrtz_job_details qrtz_job_details_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_job_details
    ADD CONSTRAINT qrtz_job_details_pkey PRIMARY KEY (sched_name, job_name, job_group);


--
-- Name: qrtz_locks qrtz_locks_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_locks
    ADD CONSTRAINT qrtz_locks_pkey PRIMARY KEY (sched_name, lock_name);


--
-- Name: qrtz_paused_trigger_grps qrtz_paused_trigger_grps_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_paused_trigger_grps
    ADD CONSTRAINT qrtz_paused_trigger_grps_pkey PRIMARY KEY (sched_name, trigger_group);


--
-- Name: qrtz_scheduler_state qrtz_scheduler_state_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_scheduler_state
    ADD CONSTRAINT qrtz_scheduler_state_pkey PRIMARY KEY (sched_name, instance_name);


--
-- Name: qrtz_simple_triggers qrtz_simple_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_simple_triggers
    ADD CONSTRAINT qrtz_simple_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);


--
-- Name: qrtz_simprop_triggers qrtz_simprop_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_simprop_triggers
    ADD CONSTRAINT qrtz_simprop_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);


--
-- Name: qrtz_triggers qrtz_triggers_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.qrtz_triggers
    ADD CONSTRAINT qrtz_triggers_pkey PRIMARY KEY (sched_name, trigger_name, trigger_group);


--
-- Name: t_ds_access_token t_ds_access_token_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_access_token
    ADD CONSTRAINT t_ds_access_token_pkey PRIMARY KEY (id);


--
-- Name: t_ds_alert t_ds_alert_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alert
    ADD CONSTRAINT t_ds_alert_pkey PRIMARY KEY (id);


--
-- Name: t_ds_alert_plugin_instance t_ds_alert_plugin_instance_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alert_plugin_instance
    ADD CONSTRAINT t_ds_alert_plugin_instance_pk PRIMARY KEY (id);


--
-- Name: t_ds_alert_send_status t_ds_alert_send_status_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alert_send_status
    ADD CONSTRAINT t_ds_alert_send_status_pkey PRIMARY KEY (id);


--
-- Name: t_ds_alertgroup t_ds_alertgroup_name_un; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alertgroup
    ADD CONSTRAINT t_ds_alertgroup_name_un UNIQUE (group_name);


--
-- Name: t_ds_alertgroup t_ds_alertgroup_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_alertgroup
    ADD CONSTRAINT t_ds_alertgroup_pkey PRIMARY KEY (id);


--
-- Name: t_ds_audit_log t_ds_audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_audit_log
    ADD CONSTRAINT t_ds_audit_log_pkey PRIMARY KEY (id);


--
-- Name: t_ds_command t_ds_command_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_command
    ADD CONSTRAINT t_ds_command_pkey PRIMARY KEY (id);


--
-- Name: t_ds_datasource t_ds_datasource_name_un; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_datasource
    ADD CONSTRAINT t_ds_datasource_name_un UNIQUE (name, type);


--
-- Name: t_ds_datasource t_ds_datasource_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_datasource
    ADD CONSTRAINT t_ds_datasource_pkey PRIMARY KEY (id);


--
-- Name: t_ds_dq_comparison_type t_ds_dq_comparison_type_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_comparison_type
    ADD CONSTRAINT t_ds_dq_comparison_type_pk PRIMARY KEY (id);


--
-- Name: t_ds_dq_execute_result t_ds_dq_execute_result_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_execute_result
    ADD CONSTRAINT t_ds_dq_execute_result_pk PRIMARY KEY (id);


--
-- Name: t_ds_dq_rule_execute_sql t_ds_dq_rule_execute_sql_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_rule_execute_sql
    ADD CONSTRAINT t_ds_dq_rule_execute_sql_pk PRIMARY KEY (id);


--
-- Name: t_ds_dq_rule_input_entry t_ds_dq_rule_input_entry_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_rule_input_entry
    ADD CONSTRAINT t_ds_dq_rule_input_entry_pk PRIMARY KEY (id);


--
-- Name: t_ds_dq_rule t_ds_dq_rule_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_rule
    ADD CONSTRAINT t_ds_dq_rule_pk PRIMARY KEY (id);


--
-- Name: t_ds_dq_task_statistics_value t_ds_dq_task_statistics_value_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_dq_task_statistics_value
    ADD CONSTRAINT t_ds_dq_task_statistics_value_pk PRIMARY KEY (id);


--
-- Name: t_ds_environment t_ds_environment_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment
    ADD CONSTRAINT t_ds_environment_pkey PRIMARY KEY (id);


--
-- Name: t_ds_environment_worker_group_relation t_ds_environment_worker_group_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_environment_worker_group_relation
    ADD CONSTRAINT t_ds_environment_worker_group_relation_pkey PRIMARY KEY (id);


--
-- Name: t_ds_error_command t_ds_error_command_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_error_command
    ADD CONSTRAINT t_ds_error_command_pkey PRIMARY KEY (id);


--
-- Name: t_ds_k8s_namespace t_ds_k8s_namespace_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_k8s_namespace
    ADD CONSTRAINT t_ds_k8s_namespace_pkey PRIMARY KEY (id);


--
-- Name: t_ds_k8s t_ds_k8s_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_k8s
    ADD CONSTRAINT t_ds_k8s_pkey PRIMARY KEY (id);


--
-- Name: t_ds_plugin_define t_ds_plugin_define_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_plugin_define
    ADD CONSTRAINT t_ds_plugin_define_pk PRIMARY KEY (id);


--
-- Name: t_ds_plugin_define t_ds_plugin_define_un; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_plugin_define
    ADD CONSTRAINT t_ds_plugin_define_un UNIQUE (plugin_name, plugin_type);


--
-- Name: t_ds_process_definition_log t_ds_process_definition_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_process_definition_log
    ADD CONSTRAINT t_ds_process_definition_log_pkey PRIMARY KEY (id);


--
-- Name: t_ds_process_definition t_ds_process_definition_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_process_definition
    ADD CONSTRAINT t_ds_process_definition_pkey PRIMARY KEY (id);


--
-- Name: t_ds_process_instance t_ds_process_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_process_instance
    ADD CONSTRAINT t_ds_process_instance_pkey PRIMARY KEY (id);


--
-- Name: t_ds_process_task_relation_log t_ds_process_task_relation_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_process_task_relation_log
    ADD CONSTRAINT t_ds_process_task_relation_log_pkey PRIMARY KEY (id);


--
-- Name: t_ds_process_task_relation t_ds_process_task_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_process_task_relation
    ADD CONSTRAINT t_ds_process_task_relation_pkey PRIMARY KEY (id);


--
-- Name: t_ds_project t_ds_project_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_project
    ADD CONSTRAINT t_ds_project_pkey PRIMARY KEY (id);


--
-- Name: t_ds_queue t_ds_queue_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_queue
    ADD CONSTRAINT t_ds_queue_pkey PRIMARY KEY (id);


--
-- Name: t_ds_relation_datasource_user t_ds_relation_datasource_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_datasource_user
    ADD CONSTRAINT t_ds_relation_datasource_user_pkey PRIMARY KEY (id);


--
-- Name: t_ds_relation_namespace_user t_ds_relation_namespace_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_namespace_user
    ADD CONSTRAINT t_ds_relation_namespace_user_pkey PRIMARY KEY (id);


--
-- Name: t_ds_relation_process_instance t_ds_relation_process_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_process_instance
    ADD CONSTRAINT t_ds_relation_process_instance_pkey PRIMARY KEY (id);


--
-- Name: t_ds_relation_project_user t_ds_relation_project_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_project_user
    ADD CONSTRAINT t_ds_relation_project_user_pkey PRIMARY KEY (id);


--
-- Name: t_ds_relation_project_user t_ds_relation_project_user_un; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_project_user
    ADD CONSTRAINT t_ds_relation_project_user_un UNIQUE (user_id, project_id);


--
-- Name: t_ds_relation_resources_user t_ds_relation_resources_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_resources_user
    ADD CONSTRAINT t_ds_relation_resources_user_pkey PRIMARY KEY (id);


--
-- Name: t_ds_relation_rule_execute_sql t_ds_relation_rule_execute_sql_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_rule_execute_sql
    ADD CONSTRAINT t_ds_relation_rule_execute_sql_pk PRIMARY KEY (id);


--
-- Name: t_ds_relation_rule_input_entry t_ds_relation_rule_input_entry_pk; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_rule_input_entry
    ADD CONSTRAINT t_ds_relation_rule_input_entry_pk PRIMARY KEY (id);


--
-- Name: t_ds_relation_udfs_user t_ds_relation_udfs_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_relation_udfs_user
    ADD CONSTRAINT t_ds_relation_udfs_user_pkey PRIMARY KEY (id);


--
-- Name: t_ds_resources t_ds_resources_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_resources
    ADD CONSTRAINT t_ds_resources_pkey PRIMARY KEY (id);


--
-- Name: t_ds_resources t_ds_resources_un; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_resources
    ADD CONSTRAINT t_ds_resources_un UNIQUE (full_name, type);


--
-- Name: t_ds_schedules t_ds_schedules_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_schedules
    ADD CONSTRAINT t_ds_schedules_pkey PRIMARY KEY (id);


--
-- Name: t_ds_session t_ds_session_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_session
    ADD CONSTRAINT t_ds_session_pkey PRIMARY KEY (id);


--
-- Name: t_ds_task_definition_log t_ds_task_definition_log_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_definition_log
    ADD CONSTRAINT t_ds_task_definition_log_pkey PRIMARY KEY (id);


--
-- Name: t_ds_task_definition t_ds_task_definition_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_definition
    ADD CONSTRAINT t_ds_task_definition_pkey PRIMARY KEY (id);


--
-- Name: t_ds_task_group t_ds_task_group_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_group
    ADD CONSTRAINT t_ds_task_group_pkey PRIMARY KEY (id);


--
-- Name: t_ds_task_group_queue t_ds_task_group_queue_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_group_queue
    ADD CONSTRAINT t_ds_task_group_queue_pkey PRIMARY KEY (id);


--
-- Name: t_ds_task_instance t_ds_task_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_instance
    ADD CONSTRAINT t_ds_task_instance_pkey PRIMARY KEY (id);


--
-- Name: t_ds_tenant t_ds_tenant_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_tenant
    ADD CONSTRAINT t_ds_tenant_pkey PRIMARY KEY (id);


--
-- Name: t_ds_udfs t_ds_udfs_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_udfs
    ADD CONSTRAINT t_ds_udfs_pkey PRIMARY KEY (id);


--
-- Name: t_ds_user t_ds_user_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_user
    ADD CONSTRAINT t_ds_user_pkey PRIMARY KEY (id);


--
-- Name: t_ds_version t_ds_version_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_version
    ADD CONSTRAINT t_ds_version_pkey PRIMARY KEY (id);


--
-- Name: t_ds_worker_group t_ds_worker_group_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_worker_group
    ADD CONSTRAINT t_ds_worker_group_pkey PRIMARY KEY (id);


--
-- Name: idx_qrtz_ft_inst_job_req_rcvry; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_ft_inst_job_req_rcvry ON public.qrtz_fired_triggers USING btree (sched_name, instance_name, requests_recovery);


--
-- Name: idx_qrtz_ft_j_g; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_ft_j_g ON public.qrtz_fired_triggers USING btree (sched_name, job_name, job_group);


--
-- Name: idx_qrtz_ft_jg; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_ft_jg ON public.qrtz_fired_triggers USING btree (sched_name, job_group);


--
-- Name: idx_qrtz_ft_t_g; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_ft_t_g ON public.qrtz_fired_triggers USING btree (sched_name, trigger_name, trigger_group);


--
-- Name: idx_qrtz_ft_tg; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_ft_tg ON public.qrtz_fired_triggers USING btree (sched_name, trigger_group);


--
-- Name: idx_qrtz_ft_trig_inst_name; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_ft_trig_inst_name ON public.qrtz_fired_triggers USING btree (sched_name, instance_name);


--
-- Name: idx_qrtz_j_grp; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_j_grp ON public.qrtz_job_details USING btree (sched_name, job_group);


--
-- Name: idx_qrtz_j_req_recovery; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_j_req_recovery ON public.qrtz_job_details USING btree (sched_name, requests_recovery);


--
-- Name: idx_qrtz_t_c; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_c ON public.qrtz_triggers USING btree (sched_name, calendar_name);


--
-- Name: idx_qrtz_t_g; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_g ON public.qrtz_triggers USING btree (sched_name, trigger_group);


--
-- Name: idx_qrtz_t_j; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_j ON public.qrtz_triggers USING btree (sched_name, job_name, job_group);


--
-- Name: idx_qrtz_t_jg; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_jg ON public.qrtz_triggers USING btree (sched_name, job_group);


--
-- Name: idx_qrtz_t_n_g_state; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_n_g_state ON public.qrtz_triggers USING btree (sched_name, trigger_group, trigger_state);


--
-- Name: idx_qrtz_t_n_state; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_n_state ON public.qrtz_triggers USING btree (sched_name, trigger_name, trigger_group, trigger_state);


--
-- Name: idx_qrtz_t_next_fire_time; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_next_fire_time ON public.qrtz_triggers USING btree (sched_name, next_fire_time);


--
-- Name: idx_qrtz_t_nft_misfire; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_nft_misfire ON public.qrtz_triggers USING btree (sched_name, misfire_instr, next_fire_time);


--
-- Name: idx_qrtz_t_nft_st; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_nft_st ON public.qrtz_triggers USING btree (sched_name, trigger_state, next_fire_time);


--
-- Name: idx_qrtz_t_nft_st_misfire; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_nft_st_misfire ON public.qrtz_triggers USING btree (sched_name, misfire_instr, next_fire_time, trigger_state);


--
-- Name: idx_qrtz_t_nft_st_misfire_grp; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_nft_st_misfire_grp ON public.qrtz_triggers USING btree (sched_name, misfire_instr, next_fire_time, trigger_group, trigger_state);


--
-- Name: idx_qrtz_t_state; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_qrtz_t_state ON public.qrtz_triggers USING btree (sched_name, trigger_state);


--
-- Name: idx_sign; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_sign ON public.t_ds_alert USING btree (sign);


--
-- Name: idx_status; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_status ON public.t_ds_alert USING btree (alert_status);


--
-- Name: idx_task_definition_log_code_version; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_task_definition_log_code_version ON public.t_ds_task_definition_log USING btree (code, version);


--
-- Name: idx_task_definition_log_project_code; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_task_definition_log_project_code ON public.t_ds_task_definition_log USING btree (project_code);


--
-- Name: idx_task_instance_code_version; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX idx_task_instance_code_version ON public.t_ds_task_instance USING btree (task_code, task_definition_version);


--
-- Name: priority_id_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX priority_id_index ON public.t_ds_command USING btree (process_instance_priority, id);


--
-- Name: process_definition_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX process_definition_index ON public.t_ds_process_definition USING btree (code, id);


--
-- Name: process_instance_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX process_instance_index ON public.t_ds_process_instance USING btree (process_definition_code, id);


--
-- Name: process_task_relation_idx_post_task_code_version; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX process_task_relation_idx_post_task_code_version ON public.t_ds_process_task_relation USING btree (post_task_code, post_task_version);


--
-- Name: process_task_relation_idx_pre_task_code_version; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX process_task_relation_idx_pre_task_code_version ON public.t_ds_process_task_relation USING btree (pre_task_code, pre_task_version);


--
-- Name: process_task_relation_idx_project_code_process_definition_code; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX process_task_relation_idx_project_code_process_definition_code ON public.t_ds_process_task_relation USING btree (project_code, process_definition_code);


--
-- Name: process_task_relation_log_idx_project_code_process_definition_c; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX process_task_relation_log_idx_project_code_process_definition_c ON public.t_ds_process_task_relation_log USING btree (project_code, process_definition_code);


--
-- Name: relation_project_user_id_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX relation_project_user_id_index ON public.t_ds_relation_project_user USING btree (user_id);


--
-- Name: start_time_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX start_time_index ON public.t_ds_process_instance USING btree (start_time, end_time);


--
-- Name: task_definition_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX task_definition_index ON public.t_ds_task_definition USING btree (project_code, id);


--
-- Name: unique_code; Type: INDEX; Schema: public; Owner: root
--

CREATE UNIQUE INDEX unique_code ON public.t_ds_project USING btree (code);


--
-- Name: unique_func_name; Type: INDEX; Schema: public; Owner: root
--

CREATE UNIQUE INDEX unique_func_name ON public.t_ds_udfs USING btree (func_name);


--
-- Name: unique_name; Type: INDEX; Schema: public; Owner: root
--

CREATE UNIQUE INDEX unique_name ON public.t_ds_project USING btree (name);


--
-- Name: unique_queue_name; Type: INDEX; Schema: public; Owner: root
--

CREATE UNIQUE INDEX unique_queue_name ON public.t_ds_queue USING btree (queue_name);


--
-- Name: unique_tenant_code; Type: INDEX; Schema: public; Owner: root
--

CREATE UNIQUE INDEX unique_tenant_code ON public.t_ds_tenant USING btree (tenant_code);


--
-- Name: user_id_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX user_id_index ON public.t_ds_project USING btree (user_id);


--
-- Name: version_index; Type: INDEX; Schema: public; Owner: root
--

CREATE INDEX version_index ON public.t_ds_version USING btree (version);


--
-- Name: t_ds_task_instance foreign_key_instance_id; Type: FK CONSTRAINT; Schema: public; Owner: root
--

ALTER TABLE ONLY public.t_ds_task_instance
    ADD CONSTRAINT foreign_key_instance_id FOREIGN KEY (process_instance_id) REFERENCES public.t_ds_process_instance(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

