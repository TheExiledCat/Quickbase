import type { Schema } from "@/classes/Schema";
import axios, { type AxiosResponse } from "axios";
import cookies from "browser-cookies";

interface ILoginTokenDto {
  token: string;
  expiration: number;
}
const adminTokenCookie = "admin_token";
const api = {
  get url() {
    return Boolean(import.meta.env.PROD)
      ? "/admin/"
      : "http://localhost:3000/admin/";
  },
  logout() {
    cookies.erase(adminTokenCookie);
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
  async getSchema(): Promise<AxiosResponse<Schema>> {
    return axios.get<Schema>(this.url + "schema");
  },
};

export default api;
