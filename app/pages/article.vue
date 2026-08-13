<script setup lang="ts">
const toast = useToast();

const post_id = ref();
const editorOptions = {
  theme: "vs-dark",
  minimap: { enabled: false },
  wordWrap: "on",
  automaticLayout: true,
} as const;
const code = ref(`---
  title:
  author:
  slug:
  category:
  tags:
  status: draft
---
# The title`);
const payload = ref<CreatePostPayload>({
  author: "",
  title: "",
  slug: "",
  category: "",
  tags: [],
  body: "",
  status: "draft",
});

const doSave = useThrottleFn(
  async () => {
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
        throw new Error("Error: Data not completed");
      }

      payload.value.author = parsedData.data.author;
      payload.value.title = parsedData.data.title;
      payload.value.slug = parsedData.data.slug;
      payload.value.category = parsedData.data.category;
      payload.value.status = parsedData.data.status;
      payload.value.tags = parsedData.data.tags.split(",");
      payload.value.body = code.value;

      if (post_id.value) {
        payload.value.id = post_id.value;
        await $fetch("/api/posts", {
          method: "PATCH",
          body: payload.value,
        });
      } else {
        const data = await $fetch("/api/posts", {
          method: "POST",
          body: payload.value,
        });
        post_id.value = data.postId;
      }

      toast.add({ title: "Data saved!", color: "success" });
    } catch (error: any) {
      toast.add({ title: error.message, color: "error" });
    }
  },
  1000,
  false,
  true,
);
</script>
<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 h-dvh">
    <div>
      <div class="text-right">
        <UButton @click="doSave()">Save</UButton>
      </div>
      <MonacoEditor
        v-model="code"
        lang="markdown"
        :options="editorOptions"
        style="height: 95%"
      />
    </div>
    <div>
      <MDC :value="code" />
      {{ payload }}
    </div>
  </div>
</template>
