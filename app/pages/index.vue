<script setup lang="ts">
import { h, resolveComponent } from "vue";
import type { TableColumn } from "@nuxt/ui";
import { invoke } from "@tauri-apps/api/core";

const UButton = resolveComponent("UButton");
const UBadge = resolveComponent("UBadge");
const UDropdownMenu = resolveComponent("UDropdownMenu");
const toast = useToast();

const reloadKey = ref(0);
const dialog = ref(false);
const items = ref<PostMetadataTable[]>();
const selectedData = ref<PostContentTable>({
  post_id: null,
  body: `---
title:
author:
slug:
category:
tags:
status: draft
---
# The title`,
});
const statuses = [
  { label: "Draft", value: "draft" },
  { label: "Published", value: "published" },
  { label: "Archived", value: "archived" },
];
const status = ref();

const columns: TableColumn<PostMetadataTable>[] = [
  {
    accessorKey: "title",
    header: "Judul",
  },
  {
    accessorKey: "status",
    header: "Status",
    cell: ({ row }) => {
      const color = {
        draft: "info" as const,
        published: "success" as const,
        archived: "neutral" as const,
      }[row.original.status as string];

      return h(UBadge, { class: "capitalize", variant: "subtle", color }, () =>
        row.getValue("status"),
      );
    },
  },
  {
    id: "actions",
    enableHiding: false,
    meta: {
      class: {
        td: "text-right",
      },
    },
    cell: ({ row }) => {
      const items = [
        {
          label: "Edit post",
          async onClick() {
            const id = row.original.id;
            if (!id) {
              return;
            }
            await getDetail(id);
            dialog.value = true;
          },
        },
      ];

      return h(
        UDropdownMenu,
        {
          content: {
            align: "end",
          },
          items,
          "aria-label": "Actions dropdown",
        },
        () =>
          h(UButton, {
            icon: "i-lucide-ellipsis-vertical",
            color: "neutral",
            variant: "ghost",
            "aria-label": "Actions dropdown",
          }),
      );
    },
  },
];

const fetchData = async () => {
  try {
    clearNuxtData("cache-table-key");
    refreshNuxtData("cache-table-key");

    const data = await invoke<PostMetadataTable[]>("get_posts");
    items.value = data;

    reloadKey.value++;
  } catch (error: any) {
    console.log(error);
    toast.add({ title: error, color: "error" });
  }
};

const getDetail = async (id: number) => {
  const data = await invoke<PostContentTable>("detail_post", { id: id });
  selectedData.value = data;
};

const resetForm = () => {
  selectedData.value = {
    post_id: null,
    body: `---
title:
author:
slug:
category:
tags:
status: draft
---
# The title`,
  };
};

fetchData();
</script>

<template>
  <div class="flex gap-2">
    <UButton
      icon="i-lucide-plus"
      label="Artikel"
      variant="soft"
      @click="
        resetForm();
        selectedData.post_id = null;
        dialog = true;
      "
    />
    <USelectMenu
      v-model="status"
      :items="statuses"
      option-attribute="name"
      placeholder="Select status"
      icon="i-heroicons-squares-2x2"
      clear
    />
  </div>

  <UTable
    :key="reloadKey"
    loading
    loading-color="primary"
    loading-animation="carousel"
    sticky
    :data="items"
    :columns="columns"
  >
    <template #title-cell="{ row }">
      <div class="font-bold text-white">{{ row.original.title }}</div>
      <div class="text-xs">
        {{ formatIdDate(row.original.created_at) }}
      </div>
      <div class="text-xs">tags: {{ row.original.tags }}</div>
      <div class="text-xs">slug: {{ row.original.slug }}</div>
    </template>
  </UTable>
  <Formarticle
    v-model:dialog="dialog"
    v-model:postid="selectedData.post_id"
    v-model:code="selectedData.body"
    @reloadparent="fetchData()"
  />
</template>
