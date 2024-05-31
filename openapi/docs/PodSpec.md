# PodSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment** | **String** | The environment where the index is hosted. | 
**replicas** | Option<**i32**> | The number of replicas. Replicas duplicate your index. They provide higher availability and throughput. Replicas can be scaled up or down as your needs change. | [optional][default to 1]
**shards** | Option<**i32**> | The number of shards. Shards split your data across multiple pods so you can fit more data into an index. | [optional][default to 1]
**pod_type** | **String** | The type of pod to use. One of `s1`, `p1`, or `p2` appended with `.` and one of `x1`, `x2`, `x4`, or `x8`. | [default to p1.x1]
**pods** | **i32** | The number of pods to be used in the index. This should be equal to `shards` x `replicas`.' | [default to 1]
**metadata_config** | Option<[**models::PodSpecMetadataConfig**](PodSpec_metadata_config.md)> |  | [optional]
**source_collection** | Option<**String**> | The name of the collection to be used as the source for the index. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


