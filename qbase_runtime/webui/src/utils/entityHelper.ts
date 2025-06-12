const fieldOrder: Record<string,number> ={
    "id": 0,
    "created": 1,
    "updated": 2,
    "username": 3,
    "email":4,
    "password":5,
}

export function GetFieldOrder(fieldName:string): number{
    return fieldOrder[fieldName] ?? Object.keys(fieldOrder).length
}