package openapi

import (
	"encoding/json"
	"fmt"
)

var oneReply = `
{
    "temperature": 88.0,
    "dayOfWeek": "monday",
    "optionalObject": {
        "sadField": "happyValue",
        "alsoOptional": "becauseWhyNot?"
    }
}
`

var otherReply = `
{
    "temperature": 88.0,
}
`

type ReplyModel struct {
    Temperature OptionalInt   `json:"temperature"`
	OptionalObj OptionalModel `json:"optionalObj"`
    set         bool          `json:"-"`
}

func (r *ReplyModel) UnmarshalJSON(data []byte) error {
    r.set = true
    // some stuff here to iterate through the keys in data
}

type OptionalModel struct {
	SadField     OptionalString `json:"sadField"`
	AlsoOptional OptionalString `json:"alsoOptional"`
}

type OptionalString struct {
	val interface{}
}

func (o *OptionalString) IsSet() bool {
    _, ok := o.val.(*string)
    return ok
}

func (o *OptionalString) UnmarshalJSON(data []byte) error {
    return json.Unmarshal(data, &o.val)
}

type OptionalInt struct {
    val interface{}
}

func (o *OptionalInt) IsSet() bool {
    _, ok := o.val.(*int)
    return ok
}

func (o *OptionalInt) UnmarshalJSON(data []byte) error {
    return json.Unmarshal(data, &o.val)
}
