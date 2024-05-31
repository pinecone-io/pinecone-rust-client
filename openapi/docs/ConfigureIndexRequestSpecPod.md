# ConfigureIndexRequestSpecPod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**replicas** | Option<**i32**> | The number of replicas. Replicas duplicate your index. They provide higher availability and throughput. Replicas can be scaled up or down as your needs change. | [optional][default to 1]
**pod_type** | Option<**String**> | The type of pod to use. One of `s1`, `p1`, or `p2` appended with `.` and one of `x1`, `x2`, `x4`, or `x8`. | [optional][default to p1.x1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


