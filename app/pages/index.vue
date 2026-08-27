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

const columns: TableColumn<PostMetadataTable>[] = [
  {
    accessorKey: "created_at",
    header: "Tgl submit",
    cell: ({ row }) => {
      return useDateFormat(
        row.getValue("created_at"),
        "dddd, DD MMM YYYY HH:mm",
        { locales: "id-ID" },
      ).value;
    },
  },
  {
    accessorKey: "title",
    header: "Judul",
  },
  {
    accessorKey: "category",
    header: "Kategori",
  },
  {
    accessorKey: "slug",
    header: "Slug",
  },
  {
    accessorKey: "tags",
    header: "Tags",
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
  <UTable
    :key="reloadKey"
    loading
    loading-color="primary"
    loading-animation="carousel"
    sticky
    :data="items"
    :columns="columns"
    class="flex-1"
  />
  <Formarticle
    v-model:dialog="dialog"
    v-model:postid="selectedData.post_id"
    v-model:code="selectedData.body"
    @reloadparent="fetchData()"
  />
</template>
