<script setup lang="ts">
import { Codemirror } from "vue-codemirror";
import { markdown } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import { oneDark } from "@codemirror/theme-one-dark";
import { invoke } from "@tauri-apps/api/core";

const toast = useToast();

const dialog = defineModel("dialog", { type: Boolean, default: false });
const post_id = defineModel("postid", { type: Number, default: null });
const code = defineModel("code", {
  type: String,
  default: "",
});
const extensions = [markdown({ codeLanguages: languages }), oneDark];

const emit = defineEmits(["reloadparent"]);

const payload = ref<CreatePostPayload>({
  author: "",
  title: "",
  slug: "",
  category: "",
  tags: [],
  body: "",
  status: "draft",
  published_at: null,
});

const getSQLiteTimestamp = (): string => {
  const now = new Date();

  const pad = (num: number) => String(num).padStart(2, "0");

  const year = now.getFullYear();
  const month = pad(now.getMonth() + 1);
  const day = pad(now.getDate());
  const hours = pad(now.getHours());
  const minutes = pad(now.getMinutes());
  const seconds = pad(now.getSeconds());

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
};

const doSave = useThrottleFn(
  async (close?: string) => {
    try {
      const parsedData = await parseMarkdown(code.value);
      if (
        !parsedData.data.title ||
        parsedData.data.title === "" ||
        !parsedData.data.author ||
        parsedData.data.author === "" ||
        !parsedData.data.slug ||
        parsedData.data.slug === "" ||
        !parsedData.data.category ||
        parsedData.data.category === "" ||
        !parsedData.data.status ||
        parsedData.data.status === "" ||
        !parsedData.data.tags ||
        parsedData.data.tags === ""
      ) {
        throw new Error(
          "No data to input: make sure title, author, slug, category, status, tags are filled",
        );
      }

      payload.value.author = parsedData.data.author;
      payload.value.title = parsedData.data.title;
      payload.value.slug = parsedData.data.slug;
      payload.value.category = parsedData.data.category;
      payload.value.status = parsedData.data.status;
      payload.value.tags = parsedData.data.tags.split(",");
      payload.value.body = code.value;
      payload.value.published_at =
        parsedData.data.published_at != null &&
        parsedData.data.status === "published"
          ? getSQLiteTimestamp()
          : null;

      if (post_id.value) {
        payload.value.id = post_id.value;
        await invoke<number>("update_post", {
          payload: payload.value,
        });
        console.log(payload.value);
      } else {
        const postId = await invoke<number>("create_post", {
          payload: payload.value,
        });
        post_id.value = postId;
      }

      toast.add({ title: "Data saved!", color: "success" });
      emit("reloadparent");
      if (close === "close") {
        dialog.value = false;
      }
    } catch (err: any) {
      toast.add({
        title: err instanceof Error ? err.message : String(err),
        color: "error",
      });
    }
  },
  1000,
  false,
  true,
);
</script>
<template>
  <UModal
    v-model:open="dialog"
    fullscreen
    :dismissible="false"
    :ui="{ body: 'p-0 sm:p-0' }"
  >
    <template #header="{ close }">
      <div class="flex items-center justify-between w-full">
        <h2 class="font-semibold text-highlighted">Artikel {{ post_id }}</h2>
        <div class="flex items-center gap-1.5">
          <UButton
            icon="i-lucide-save-all"
            color="neutral"
            variant="ghost"
            @click="doSave('close')"
            label="Finish"
          />
          <UButton
            icon="i-lucide-refresh-ccw"
            color="neutral"
            variant="ghost"
            @click="doSave()"
            label="Update"
          />
          <UButton
            icon="i-lucide-x"
            color="neutral"
            variant="ghost"
            @click="close"
          />
        </div>
      </div>
    </template>
    <template #body>
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 h-dvh">
        <div>
          <ClientOnly fallback="Loading Markdown Editor...">
            <Codemirror
              v-model="code"
              :style="{ height: '100vh' }"
              :extensions="extensions"
            />
          </ClientOnly>
        </div>
        <div>
          <MDC :value="code" :key="code" />
        </div>
      </div>
    </template>
  </UModal>
</template>
