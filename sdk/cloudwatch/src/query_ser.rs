// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_dimension(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Dimension,
) {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_metric_data_query(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::MetricDataQuery,
) {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Id");
    if let Some(var_6) = &input.id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MetricStat");
    if let Some(var_8) = &input.metric_stat {
        crate::query_ser::serialize_structure_metric_stat(scope_7, var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Expression");
    if let Some(var_10) = &input.expression {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Label");
    if let Some(var_12) = &input.label {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("ReturnData");
    if let Some(var_14) = &input.return_data {
        scope_13.boolean(*var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Period");
    if let Some(var_16) = &input.period {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_16).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_label_options(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::LabelOptions,
) {
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Timezone");
    if let Some(var_18) = &input.timezone {
        scope_17.string(var_18);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_dimension_filter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::DimensionFilter,
) {
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Name");
    if let Some(var_20) = &input.name {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Value");
    if let Some(var_22) = &input.value {
        scope_21.string(var_22);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_anomaly_detector_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::AnomalyDetectorConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("ExcludedTimeRanges");
    if let Some(var_24) = &input.excluded_time_ranges {
        let mut list_26 = scope_23.start_list(false, None);
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            crate::query_ser::serialize_structure_range(entry_27, item_25);
        }
        list_26.finish();
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("MetricTimezone");
    if let Some(var_29) = &input.metric_timezone {
        scope_28.string(var_29);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_tag(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) {
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("Key");
    if let Some(var_31) = &input.key {
        scope_30.string(var_31);
    }
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("Value");
    if let Some(var_33) = &input.value {
        scope_32.string(var_33);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_metric_datum(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::MetricDatum,
) {
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("MetricName");
    if let Some(var_35) = &input.metric_name {
        scope_34.string(var_35);
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("Dimensions");
    if let Some(var_37) = &input.dimensions {
        let mut list_39 = scope_36.start_list(false, None);
        for item_38 in var_37 {
            #[allow(unused_mut)]
            let mut entry_40 = list_39.entry();
            crate::query_ser::serialize_structure_dimension(entry_40, item_38);
        }
        list_39.finish();
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("Timestamp");
    if let Some(var_42) = &input.timestamp {
        scope_41.instant(var_42, smithy_types::instant::Format::DateTime);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("Value");
    if let Some(var_44) = &input.value {
        scope_43.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_44).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("StatisticValues");
    if let Some(var_46) = &input.statistic_values {
        crate::query_ser::serialize_structure_statistic_set(scope_45, var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("Values");
    if let Some(var_48) = &input.values {
        let mut list_50 = scope_47.start_list(false, None);
        for item_49 in var_48 {
            #[allow(unused_mut)]
            let mut entry_51 = list_50.entry();
            entry_51.number(
                #[allow(clippy::useless_conversion)]
                smithy_types::Number::Float((*item_49).into()),
            );
        }
        list_50.finish();
    }
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("Counts");
    if let Some(var_53) = &input.counts {
        let mut list_55 = scope_52.start_list(false, None);
        for item_54 in var_53 {
            #[allow(unused_mut)]
            let mut entry_56 = list_55.entry();
            entry_56.number(
                #[allow(clippy::useless_conversion)]
                smithy_types::Number::Float((*item_54).into()),
            );
        }
        list_55.finish();
    }
    #[allow(unused_mut)]
    let mut scope_57 = writer.prefix("Unit");
    if let Some(var_58) = &input.unit {
        scope_57.string(var_58.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("StorageResolution");
    if let Some(var_60) = &input.storage_resolution {
        scope_59.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_60).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_metric_stream_filter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::MetricStreamFilter,
) {
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("Namespace");
    if let Some(var_62) = &input.namespace {
        scope_61.string(var_62);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_metric_stat(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::MetricStat,
) {
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("Metric");
    if let Some(var_64) = &input.metric {
        crate::query_ser::serialize_structure_metric(scope_63, var_64);
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("Period");
    if let Some(var_66) = &input.period {
        scope_65.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_66).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("Stat");
    if let Some(var_68) = &input.stat {
        scope_67.string(var_68);
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("Unit");
    if let Some(var_70) = &input.unit {
        scope_69.string(var_70.as_str());
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_range(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Range,
) {
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("StartTime");
    if let Some(var_72) = &input.start_time {
        scope_71.instant(var_72, smithy_types::instant::Format::DateTime);
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("EndTime");
    if let Some(var_74) = &input.end_time {
        scope_73.instant(var_74, smithy_types::instant::Format::DateTime);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_statistic_set(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::StatisticSet,
) {
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("SampleCount");
    if let Some(var_76) = &input.sample_count {
        scope_75.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_76).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("Sum");
    if let Some(var_78) = &input.sum {
        scope_77.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_78).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("Minimum");
    if let Some(var_80) = &input.minimum {
        scope_79.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_80).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("Maximum");
    if let Some(var_82) = &input.maximum {
        scope_81.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_82).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_metric(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Metric,
) {
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("Namespace");
    if let Some(var_84) = &input.namespace {
        scope_83.string(var_84);
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("MetricName");
    if let Some(var_86) = &input.metric_name {
        scope_85.string(var_86);
    }
    #[allow(unused_mut)]
    let mut scope_87 = writer.prefix("Dimensions");
    if let Some(var_88) = &input.dimensions {
        let mut list_90 = scope_87.start_list(false, None);
        for item_89 in var_88 {
            #[allow(unused_mut)]
            let mut entry_91 = list_90.entry();
            crate::query_ser::serialize_structure_dimension(entry_91, item_89);
        }
        list_90.finish();
    }
}
