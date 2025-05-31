import axios from "axios";
import cookies from 'browser-cookies'

interface ILoginTokenDto{
    token:string,
    expiration: number
}
const api = {
    
    get url() {
        return "http://localhost:3000/admin/";
    },
    async login(identifier:string,password:string): Promise<boolean>{
        try{
            const res = await axios.post<ILoginTokenDto>(this.url+"auth",{identifier,password})
        
            cookies.set("admin_token",res.data.token, { expires:  new Date(res.data.expiration*1000)});
                return true;
        }catch{
            return false;
        }
    }
}

export default api;