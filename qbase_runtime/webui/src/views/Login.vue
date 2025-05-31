<script setup lang="ts">
import {
  Card,
  IftaLabel,
  InputText,
  Button,
  useToast,
  InputGroup,
} from "primevue";
import { ref } from "vue";
import api from "@/utils/admin_utils";
import { useRouter } from "vue-router";
const identifier = ref();
const password = ref();
const router = useRouter();
const toaster = useToast();
function tryLogin() {
  api.login(identifier.value, password.value).then((success) => {
    if (success) {
      console.log("logged in");
      router.push("/dashboard");
    } else {
      toaster.add({
        life: 3000,
        summary: "Incorrect login details",
      });
    }
  });
}
</script>
<template>
  <div class="login">
    <Card>
      <template #header>Quickbase Admin Login</template>
      <template #content>
        <IftaLabel>
          <InputText
            type="text"
            id="identifier"
            v-model="identifier"
          ></InputText>
          <label for="identifier">Username or Email</label>
        </IftaLabel>
        <IftaLabel>
          <InputText
            type="password"
            id="password"
            v-model="password"
          ></InputText>
          <label for="password">Password</label>
        </IftaLabel>
        <InputGroup>
          <Button @click="tryLogin">Login</Button>
        </InputGroup>
      </template>
    </Card>
  </div>
</template>

<style scoped>
.login {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100vh;
}
</style>
