package main

import (
    "fmt"
    "encoding/json"
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

// OptionalInt wraps a JSON Object's leaf properties
// allowing to see if they've been set
type OptionalInt struct {
    val interface{}
}

// NewOptionalInt returns a new optional int
func NewOptionalInt(val int) OptionalInt {
    innerVal := new(int)
    *innerVal = val
    return OptionalInt{innerVal}
}

// IsSet lets us check if the field was set
func (o *OptionalInt) IsSet() bool {
    _, isInt := o.val.(*int)
    return isInt
}

// UnmarshalJSON implements json.Unmarshaller for OptionalInt
func (o *OptionalInt) UnmarshalJSON(data []byte) error {
    return json.Unmarshal(data, &o.val)
}

func main() {
    o := OptionalInt{}
    thing := o.val.(*int)
    *thing = *thing + 1 * 90
    o.val = thing
    val, isIntPtr := o.val.(*int)
	fmt.Println(val, isIntPtr)
}
