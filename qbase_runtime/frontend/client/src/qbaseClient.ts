import QueryableEntity from "./entities/entityQueryable";

class QbaseClient{
    url:string;
    
    constructor(url:string){
        this.url = url;
    }

    entity(entityName:string){
        return new QueryableEntity(entityName);
    }
}
export default QbaseClient; 