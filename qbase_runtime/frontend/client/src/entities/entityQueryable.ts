import axios from "axios";
import QbaseClient from "../qbaseClient";
import EntityRecord from "../../../"
export default class QueryableEntity{
    entityName:string
    baseClient:QbaseClient
    constructor(entityName:string, client:QbaseClient
    
    ){
        this.entityName = entityName;
        this.baseClient = client;
    }

    find(id:string):Promise<EntityRecord>{
        axios.get(`${this.baseClient.url}entities/${this.entityName}/${id}`);
    }
}