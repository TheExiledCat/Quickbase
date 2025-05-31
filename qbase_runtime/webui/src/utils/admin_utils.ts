import axios from "axios";
import cookies from "browser-cookies";

interface ILoginTokenDto {
  token: string;
  expiration: number;
}
const adminTokenCookie = "admin_token";
const api = {
  get url() {
    return "http://localhost:3000/admin/";
  },
  async login(identifier: string, password: string): Promise<boolean> {
    try {
      const res = await axios.post<ILoginTokenDto>(this.url + "auth", {
        identifier,
        password,
      });

      cookies.set(adminTokenCookie, res.data.token, {
        expires: new Date(res.data.expiration * 1000),
      });
      return true;
    } catch {
      return false;
    }
  },
  isLoggedIn(): boolean {
    return Boolean(cookies.get(adminTokenCookie));
  },
};

export default api;
