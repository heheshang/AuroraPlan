// @generated automatically by Diesel CLI.

diesel::table! {
    qrtz_blob_triggers (sched_name, trigger_name, trigger_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        trigger_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
        blob_data -> Nullable<Bytea>,
    }
}

diesel::table! {
    qrtz_calendars (sched_name, calendar_name) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        calendar_name -> Varchar,
        calendar -> Bytea,
    }
}

diesel::table! {
    qrtz_cron_triggers (sched_name, trigger_name, trigger_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        trigger_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
        #[max_length = 120]
        cron_expression -> Varchar,
        #[max_length = 80]
        time_zone_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    qrtz_fired_triggers (sched_name, entry_id) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        entry_id -> Varchar,
        #[max_length = 200]
        trigger_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
        #[max_length = 200]
        instance_name -> Varchar,
        fired_time -> Int8,
        sched_time -> Int8,
        priority -> Int4,
        #[max_length = 16]
        state -> Varchar,
        #[max_length = 200]
        job_name -> Nullable<Varchar>,
        #[max_length = 200]
        job_group -> Nullable<Varchar>,
        is_nonconcurrent -> Nullable<Bool>,
        requests_recovery -> Nullable<Bool>,
    }
}

diesel::table! {
    qrtz_job_details (sched_name, job_name, job_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        job_name -> Varchar,
        #[max_length = 200]
        job_group -> Varchar,
        #[max_length = 250]
        description -> Nullable<Varchar>,
        #[max_length = 250]
        job_class_name -> Varchar,
        is_durable -> Bool,
        is_nonconcurrent -> Bool,
        is_update_data -> Bool,
        requests_recovery -> Bool,
        job_data -> Nullable<Bytea>,
    }
}

diesel::table! {
    qrtz_locks (sched_name, lock_name) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 40]
        lock_name -> Varchar,
    }
}

diesel::table! {
    qrtz_paused_trigger_grps (sched_name, trigger_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
    }
}

diesel::table! {
    qrtz_scheduler_state (sched_name, instance_name) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        instance_name -> Varchar,
        last_checkin_time -> Int8,
        checkin_interval -> Int8,
    }
}

diesel::table! {
    qrtz_simple_triggers (sched_name, trigger_name, trigger_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        trigger_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
        repeat_count -> Int8,
        repeat_interval -> Int8,
        times_triggered -> Int8,
    }
}

diesel::table! {
    qrtz_simprop_triggers (sched_name, trigger_name, trigger_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        trigger_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
        #[max_length = 512]
        str_prop_1 -> Nullable<Varchar>,
        #[max_length = 512]
        str_prop_2 -> Nullable<Varchar>,
        #[max_length = 512]
        str_prop_3 -> Nullable<Varchar>,
        int_prop_1 -> Nullable<Int4>,
        int_prop_2 -> Nullable<Int4>,
        long_prop_1 -> Nullable<Int8>,
        long_prop_2 -> Nullable<Int8>,
        dec_prop_1 -> Numeric,
        dec_prop_2 -> Numeric,
        bool_prop_1 -> Nullable<Bool>,
        bool_prop_2 -> Nullable<Bool>,
    }
}

diesel::table! {
    qrtz_triggers (sched_name, trigger_name, trigger_group) {
        #[max_length = 120]
        sched_name -> Varchar,
        #[max_length = 200]
        trigger_name -> Varchar,
        #[max_length = 200]
        trigger_group -> Varchar,
        #[max_length = 200]
        job_name -> Varchar,
        #[max_length = 200]
        job_group -> Varchar,
        #[max_length = 250]
        description -> Nullable<Varchar>,
        next_fire_time -> Nullable<Int8>,
        prev_fire_time -> Nullable<Int8>,
        priority -> Nullable<Int4>,
        #[max_length = 16]
        trigger_state -> Varchar,
        #[max_length = 8]
        trigger_type -> Varchar,
        start_time -> Int8,
        end_time -> Nullable<Int8>,
        #[max_length = 200]
        calendar_name -> Nullable<Varchar>,
        misfire_instr -> Nullable<Int2>,
        job_data -> Nullable<Bytea>,
    }
}

diesel::table! {
    t_ds_access_token (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 64]
        token -> Nullable<Varchar>,
        expire_time -> Nullable<Timestamp>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_alert (id) {
        id -> Int4,
        #[max_length = 512]
        title -> Nullable<Varchar>,
        #[max_length = 40]
        sign -> Varchar,
        content -> Nullable<Text>,
        alert_status -> Nullable<Int4>,
        warning_type -> Nullable<Int4>,
        log -> Nullable<Text>,
        alertgroup_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        project_code -> Nullable<Int8>,
        process_definition_code -> Nullable<Int8>,
        process_instance_id -> Nullable<Int4>,
        alert_type -> Nullable<Int4>,
    }
}

diesel::table! {
    t_ds_alert_plugin_instance (id) {
        id -> Int4,
        plugin_define_id -> Int4,
        plugin_instance_params -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        #[max_length = 255]
        instance_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    t_ds_alert_send_status (id) {
        id -> Int4,
        alert_id -> Int4,
        alert_plugin_instance_id -> Int4,
        send_status -> Nullable<Int4>,
        log -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_alertgroup (id) {
        id -> Int4,
        #[max_length = 255]
        alert_instance_ids -> Nullable<Varchar>,
        create_user_id -> Nullable<Int4>,
        #[max_length = 255]
        group_name -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_audit_log (id) {
        id -> Int4,
        user_id -> Int4,
        resource_type -> Int4,
        operation -> Int4,
        time -> Nullable<Timestamp>,
        resource_id -> Int4,
    }
}

diesel::table! {
    t_ds_cluster (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        config -> Nullable<Text>,
        description -> Nullable<Text>,
        operator -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_command (id) {
        id -> Int4,
        command_type -> Nullable<Int4>,
        process_definition_code -> Int8,
        command_param -> Nullable<Text>,
        task_depend_type -> Nullable<Int4>,
        failure_strategy -> Nullable<Int4>,
        warning_type -> Nullable<Int4>,
        warning_group_id -> Nullable<Int4>,
        schedule_time -> Nullable<Timestamp>,
        start_time -> Nullable<Timestamp>,
        executor_id -> Nullable<Int4>,
        update_time -> Nullable<Timestamp>,
        process_instance_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        #[max_length = 64]
        tenant_code -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        dry_run -> Nullable<Int4>,
        process_instance_id -> Nullable<Int4>,
        process_definition_version -> Nullable<Int4>,
        test_flag -> Nullable<Int4>,
    }
}

diesel::table! {
    t_ds_datasource (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 255]
        note -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Int4,
        user_id -> Int4,
        connection_params -> Text,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_dq_comparison_type (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        execute_sql -> Nullable<Varchar>,
        output_table -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        is_inner_source -> Nullable<Bool>,
    }
}

diesel::table! {
    t_ds_dq_execute_result (id) {
        id -> Int4,
        process_definition_id -> Nullable<Int4>,
        process_instance_id -> Nullable<Int4>,
        task_instance_id -> Nullable<Int4>,
        rule_type -> Nullable<Int4>,
        #[max_length = 255]
        rule_name -> Nullable<Varchar>,
        statistics_value -> Nullable<Float8>,
        comparison_value -> Nullable<Float8>,
        check_type -> Nullable<Int4>,
        threshold -> Nullable<Float8>,
        operator -> Nullable<Int4>,
        failure_strategy -> Nullable<Int4>,
        state -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        comparison_type -> Nullable<Int4>,
        error_output_path -> Nullable<Text>,
    }
}

diesel::table! {
    t_ds_dq_rule (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_dq_rule_execute_sql (id) {
        id -> Int4,
        index -> Nullable<Int4>,
        sql -> Nullable<Text>,
        #[max_length = 255]
        table_alias -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        is_error_output_sql -> Nullable<Bool>,
    }
}

diesel::table! {
    t_ds_dq_rule_input_entry (id) {
        id -> Int4,
        #[max_length = 255]
        field -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        value -> Nullable<Varchar>,
        options -> Nullable<Text>,
        #[max_length = 255]
        placeholder -> Nullable<Varchar>,
        option_source_type -> Nullable<Int4>,
        value_type -> Nullable<Int4>,
        input_type -> Nullable<Int4>,
        is_show -> Nullable<Int2>,
        can_edit -> Nullable<Int2>,
        is_emit -> Nullable<Int2>,
        is_validate -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_dq_task_statistics_value (id) {
        id -> Int4,
        process_definition_id -> Int4,
        task_instance_id -> Nullable<Int4>,
        rule_id -> Int4,
        unique_code -> Varchar,
        statistics_name -> Nullable<Varchar>,
        statistics_value -> Nullable<Float8>,
        data_time -> Nullable<Timestamp>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_environment (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        config -> Nullable<Text>,
        description -> Nullable<Text>,
        operator -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_environment_relation (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        config -> Nullable<Text>,
        description -> Nullable<Text>,
        operator -> Nullable<Int4>,
        worker_groups -> Nullable<Array<Nullable<Text>>>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_environment_worker_group_relation (id) {
        id -> Int4,
        environment_code -> Int8,
        #[max_length = 255]
        worker_group -> Varchar,
        operator -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_error_command (id) {
        id -> Int4,
        command_type -> Nullable<Int4>,
        process_definition_code -> Int8,
        command_param -> Nullable<Text>,
        task_depend_type -> Nullable<Int4>,
        failure_strategy -> Nullable<Int4>,
        warning_type -> Nullable<Int4>,
        warning_group_id -> Nullable<Int4>,
        schedule_time -> Nullable<Timestamp>,
        start_time -> Nullable<Timestamp>,
        executor_id -> Nullable<Int4>,
        update_time -> Nullable<Timestamp>,
        process_instance_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        #[max_length = 64]
        tenant_code -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        dry_run -> Nullable<Int4>,
        message -> Nullable<Text>,
        process_instance_id -> Nullable<Int4>,
        process_definition_version -> Nullable<Int4>,
        test_flag -> Nullable<Int4>,
    }
}

diesel::table! {
    t_ds_fav_task (id) {
        id -> Int4,
        #[max_length = 64]
        task_type -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    t_ds_k8s (id) {
        id -> Int4,
        #[max_length = 255]
        k8s_name -> Nullable<Varchar>,
        k8s_config -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_k8s_namespace (id) {
        id -> Int4,
        code -> Int8,
        limits_memory -> Nullable<Int4>,
        #[max_length = 255]
        namespace -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        pod_replicas -> Nullable<Int4>,
        pod_request_cpu -> Nullable<Numeric>,
        pod_request_memory -> Nullable<Int4>,
        limits_cpu -> Nullable<Numeric>,
        cluster_code -> Int8,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_plugin_define (id) {
        id -> Int4,
        #[max_length = 255]
        plugin_name -> Varchar,
        #[max_length = 63]
        plugin_type -> Varchar,
        plugin_params -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_process_definition (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        version -> Int4,
        description -> Nullable<Text>,
        project_code -> Nullable<Int8>,
        release_state -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        global_params -> Nullable<Text>,
        locations -> Nullable<Text>,
        warning_group_id -> Nullable<Int4>,
        flag -> Nullable<Int4>,
        timeout -> Nullable<Int4>,
        execution_type -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_process_definition_log (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        version -> Int4,
        description -> Nullable<Text>,
        project_code -> Nullable<Int8>,
        release_state -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        global_params -> Nullable<Text>,
        locations -> Nullable<Text>,
        warning_group_id -> Nullable<Int4>,
        flag -> Nullable<Int4>,
        timeout -> Nullable<Int4>,
        execution_type -> Nullable<Int4>,
        operator -> Nullable<Int4>,
        operate_time -> Nullable<Timestamp>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_process_instance (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        process_definition_code -> Nullable<Int8>,
        process_definition_version -> Nullable<Int4>,
        project_code -> Nullable<Int8>,
        state -> Nullable<Int4>,
        state_history -> Nullable<Text>,
        recovery -> Nullable<Int4>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        run_times -> Nullable<Int4>,
        #[max_length = 135]
        host -> Nullable<Varchar>,
        command_type -> Nullable<Int4>,
        command_param -> Nullable<Text>,
        task_depend_type -> Nullable<Int4>,
        max_try_times -> Nullable<Int4>,
        failure_strategy -> Nullable<Int4>,
        warning_type -> Nullable<Int4>,
        warning_group_id -> Nullable<Int4>,
        schedule_time -> Nullable<Timestamp>,
        command_start_time -> Nullable<Timestamp>,
        global_params -> Nullable<Text>,
        process_instance_json -> Nullable<Text>,
        flag -> Nullable<Int4>,
        update_time -> Nullable<Timestamp>,
        is_sub_process -> Nullable<Int4>,
        executor_id -> Int4,
        #[max_length = 64]
        executor_name -> Nullable<Varchar>,
        history_cmd -> Nullable<Text>,
        dependence_schedule_times -> Nullable<Text>,
        process_instance_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        timeout -> Nullable<Int4>,
        #[max_length = 64]
        tenant_code -> Nullable<Varchar>,
        var_pool -> Nullable<Text>,
        dry_run -> Nullable<Int4>,
        next_process_instance_id -> Nullable<Int4>,
        restart_time -> Nullable<Timestamp>,
        test_flag -> Nullable<Int4>,
    }
}

diesel::table! {
    t_ds_process_task_relation (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        project_code -> Nullable<Int8>,
        process_definition_code -> Nullable<Int8>,
        process_definition_version -> Nullable<Int4>,
        pre_task_code -> Nullable<Int8>,
        pre_task_version -> Nullable<Int4>,
        post_task_code -> Nullable<Int8>,
        post_task_version -> Nullable<Int4>,
        condition_type -> Nullable<Int4>,
        condition_params -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_process_task_relation_log (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        project_code -> Nullable<Int8>,
        process_definition_code -> Nullable<Int8>,
        process_definition_version -> Nullable<Int4>,
        pre_task_code -> Nullable<Int8>,
        pre_task_version -> Nullable<Int4>,
        post_task_code -> Nullable<Int8>,
        post_task_version -> Nullable<Int4>,
        condition_type -> Nullable<Int4>,
        condition_params -> Nullable<Text>,
        operator -> Nullable<Int4>,
        operate_time -> Nullable<Timestamp>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_project (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        code -> Int8,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        flag -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_project_parameter (id) {
        id -> Int4,
        #[max_length = 255]
        param_name -> Varchar,
        #[max_length = 255]
        param_value -> Varchar,
        code -> Int8,
        project_code -> Int8,
        user_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_project_preference (id) {
        id -> Int4,
        code -> Int8,
        project_code -> Int8,
        #[max_length = 512]
        preferences -> Varchar,
        user_id -> Nullable<Int4>,
        state -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_queue (id) {
        id -> Int4,
        #[max_length = 64]
        queue_name -> Nullable<Varchar>,
        #[max_length = 64]
        queue -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_datasource_user (id) {
        id -> Int4,
        user_id -> Int4,
        datasource_id -> Nullable<Int4>,
        perm -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_namespace_user (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        namespace_id -> Nullable<Int4>,
        perm -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_process_instance (id) {
        id -> Int4,
        parent_process_instance_id -> Nullable<Int4>,
        parent_task_instance_id -> Nullable<Int4>,
        process_instance_id -> Nullable<Int4>,
    }
}

diesel::table! {
    t_ds_relation_project_user (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Nullable<Int4>,
        perm -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_resources_user (id) {
        id -> Int4,
        user_id -> Int4,
        resources_id -> Nullable<Int4>,
        perm -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_rule_execute_sql (id) {
        id -> Int4,
        rule_id -> Nullable<Int4>,
        execute_sql_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_rule_input_entry (id) {
        id -> Int4,
        rule_id -> Nullable<Int4>,
        rule_input_entry_id -> Nullable<Int4>,
        values_map -> Nullable<Text>,
        index -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_relation_sub_workflow (id) {
        id -> Int4,
        parent_workflow_instance_id -> Int8,
        parent_task_code -> Int8,
        sub_workflow_instance_id -> Int8,
    }
}

diesel::table! {
    t_ds_relation_udfs_user (id) {
        id -> Int4,
        user_id -> Int4,
        udf_id -> Nullable<Int4>,
        perm -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_resources (id) {
        id -> Int4,
        #[max_length = 64]
        alias -> Nullable<Varchar>,
        #[max_length = 64]
        file_name -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Int4>,
        size -> Nullable<Int8>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        pid -> Nullable<Int4>,
        #[max_length = 128]
        full_name -> Nullable<Varchar>,
        is_directory -> Nullable<Bool>,
    }
}

diesel::table! {
    t_ds_schedules (id) {
        id -> Int4,
        process_definition_code -> Int8,
        start_time -> Timestamp,
        end_time -> Timestamp,
        #[max_length = 40]
        timezone_id -> Nullable<Varchar>,
        #[max_length = 255]
        crontab -> Varchar,
        failure_strategy -> Int4,
        user_id -> Int4,
        release_state -> Int4,
        warning_type -> Int4,
        warning_group_id -> Nullable<Int4>,
        process_instance_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        #[max_length = 64]
        tenant_code -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    t_ds_session (id) {
        #[max_length = 64]
        id -> Varchar,
        user_id -> Nullable<Int4>,
        #[max_length = 45]
        ip -> Nullable<Varchar>,
        last_login_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_task_definition (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        version -> Int4,
        description -> Nullable<Text>,
        project_code -> Nullable<Int8>,
        user_id -> Nullable<Int4>,
        #[max_length = 50]
        task_type -> Nullable<Varchar>,
        task_execute_type -> Nullable<Int4>,
        task_params -> Nullable<Text>,
        flag -> Nullable<Int4>,
        is_cache -> Nullable<Int4>,
        task_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        fail_retry_times -> Nullable<Int4>,
        fail_retry_interval -> Nullable<Int4>,
        timeout_flag -> Nullable<Int4>,
        timeout_notify_strategy -> Nullable<Int4>,
        timeout -> Nullable<Int4>,
        delay_time -> Nullable<Int4>,
        task_group_id -> Nullable<Int4>,
        task_group_priority -> Nullable<Int4>,
        resource_ids -> Nullable<Text>,
        cpu_quota -> Int4,
        memory_max -> Int4,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_task_definition_log (id) {
        id -> Int4,
        code -> Int8,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        version -> Int4,
        description -> Nullable<Text>,
        project_code -> Nullable<Int8>,
        user_id -> Nullable<Int4>,
        #[max_length = 50]
        task_type -> Nullable<Varchar>,
        task_execute_type -> Nullable<Int4>,
        task_params -> Nullable<Text>,
        flag -> Nullable<Int4>,
        is_cache -> Nullable<Int4>,
        task_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        fail_retry_times -> Nullable<Int4>,
        fail_retry_interval -> Nullable<Int4>,
        timeout_flag -> Nullable<Int4>,
        timeout_notify_strategy -> Nullable<Int4>,
        timeout -> Nullable<Int4>,
        delay_time -> Nullable<Int4>,
        resource_ids -> Nullable<Text>,
        operator -> Nullable<Int4>,
        task_group_id -> Nullable<Int4>,
        task_group_priority -> Nullable<Int4>,
        operate_time -> Nullable<Timestamp>,
        cpu_quota -> Int4,
        memory_max -> Int4,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_task_group (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        group_size -> Int4,
        project_code -> Nullable<Int8>,
        use_size -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        status -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_task_group_queue (id) {
        id -> Int4,
        task_id -> Nullable<Int4>,
        #[max_length = 255]
        task_name -> Nullable<Varchar>,
        group_id -> Nullable<Int4>,
        process_id -> Nullable<Int4>,
        priority -> Nullable<Int4>,
        status -> Nullable<Int4>,
        force_start -> Nullable<Int4>,
        in_queue -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_task_instance (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        task_type -> Nullable<Varchar>,
        task_execute_type -> Nullable<Int4>,
        task_code -> Int8,
        task_definition_version -> Nullable<Int4>,
        process_instance_id -> Nullable<Int4>,
        #[max_length = 255]
        process_instance_name -> Nullable<Varchar>,
        project_code -> Nullable<Int8>,
        state -> Nullable<Int4>,
        submit_time -> Nullable<Timestamp>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        #[max_length = 135]
        host -> Nullable<Varchar>,
        #[max_length = 200]
        execute_path -> Nullable<Varchar>,
        log_path -> Nullable<Text>,
        alert_flag -> Nullable<Int4>,
        retry_times -> Nullable<Int4>,
        pid -> Nullable<Int4>,
        app_link -> Nullable<Text>,
        task_params -> Nullable<Text>,
        flag -> Nullable<Int4>,
        is_cache -> Nullable<Int4>,
        #[max_length = 200]
        cache_key -> Nullable<Varchar>,
        retry_interval -> Nullable<Int4>,
        max_retry_times -> Nullable<Int4>,
        task_instance_priority -> Nullable<Int4>,
        #[max_length = 255]
        worker_group -> Nullable<Varchar>,
        environment_code -> Nullable<Int8>,
        environment_config -> Nullable<Text>,
        executor_id -> Nullable<Int4>,
        #[max_length = 64]
        executor_name -> Nullable<Varchar>,
        first_submit_time -> Nullable<Timestamp>,
        delay_time -> Nullable<Int4>,
        task_group_id -> Nullable<Int4>,
        var_pool -> Nullable<Text>,
        dry_run -> Nullable<Int4>,
        cpu_quota -> Int4,
        memory_max -> Int4,
        test_flag -> Nullable<Int4>,
    }
}

diesel::table! {
    t_ds_tenant (id) {
        id -> Int4,
        #[max_length = 64]
        tenant_code -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        queue_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_trigger_relation (id) {
        id -> Int4,
        trigger_type -> Int4,
        trigger_code -> Int8,
        job_id -> Int8,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    t_ds_udfs (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        func_name -> Varchar,
        #[max_length = 255]
        class_name -> Varchar,
        #[sql_name = "type"]
        type_ -> Int4,
        #[max_length = 255]
        arg_types -> Nullable<Varchar>,
        #[max_length = 255]
        database -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        resource_id -> Int4,
        #[max_length = 255]
        resource_name -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    t_ds_user (id) {
        id -> Int4,
        #[max_length = 64]
        user_name -> Nullable<Varchar>,
        #[max_length = 64]
        user_password -> Nullable<Varchar>,
        user_type -> Nullable<Int4>,
        #[max_length = 64]
        email -> Nullable<Varchar>,
        #[max_length = 11]
        phone -> Nullable<Varchar>,
        tenant_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        #[max_length = 64]
        queue -> Nullable<Varchar>,
        state -> Nullable<Int4>,
        #[max_length = 32]
        time_zone -> Nullable<Varchar>,
    }
}

diesel::table! {
    t_ds_version (id) {
        id -> Int4,
        #[max_length = 63]
        version -> Varchar,
    }
}

diesel::table! {
    t_ds_worker_group (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        addr_list -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        other_params_json -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    qrtz_blob_triggers,
    qrtz_calendars,
    qrtz_cron_triggers,
    qrtz_fired_triggers,
    qrtz_job_details,
    qrtz_locks,
    qrtz_paused_trigger_grps,
    qrtz_scheduler_state,
    qrtz_simple_triggers,
    qrtz_simprop_triggers,
    qrtz_triggers,
    t_ds_access_token,
    t_ds_alert,
    t_ds_alert_plugin_instance,
    t_ds_alert_send_status,
    t_ds_alertgroup,
    t_ds_audit_log,
    t_ds_cluster,
    t_ds_command,
    t_ds_datasource,
    t_ds_dq_comparison_type,
    t_ds_dq_execute_result,
    t_ds_dq_rule,
    t_ds_dq_rule_execute_sql,
    t_ds_dq_rule_input_entry,
    t_ds_dq_task_statistics_value,
    t_ds_environment,
    t_ds_environment_relation,
    t_ds_environment_worker_group_relation,
    t_ds_error_command,
    t_ds_fav_task,
    t_ds_k8s,
    t_ds_k8s_namespace,
    t_ds_plugin_define,
    t_ds_process_definition,
    t_ds_process_definition_log,
    t_ds_process_instance,
    t_ds_process_task_relation,
    t_ds_process_task_relation_log,
    t_ds_project,
    t_ds_project_parameter,
    t_ds_project_preference,
    t_ds_queue,
    t_ds_relation_datasource_user,
    t_ds_relation_namespace_user,
    t_ds_relation_process_instance,
    t_ds_relation_project_user,
    t_ds_relation_resources_user,
    t_ds_relation_rule_execute_sql,
    t_ds_relation_rule_input_entry,
    t_ds_relation_sub_workflow,
    t_ds_relation_udfs_user,
    t_ds_resources,
    t_ds_schedules,
    t_ds_session,
    t_ds_task_definition,
    t_ds_task_definition_log,
    t_ds_task_group,
    t_ds_task_group_queue,
    t_ds_task_instance,
    t_ds_tenant,
    t_ds_trigger_relation,
    t_ds_udfs,
    t_ds_user,
    t_ds_version,
    t_ds_worker_group,
);
