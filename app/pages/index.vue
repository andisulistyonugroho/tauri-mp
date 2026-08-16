<script setup lang="ts">
import { h, resolveComponent } from "vue";
import type { TableColumn } from "@nuxt/ui";

const UButton = resolveComponent("UButton");
const UBadge = resolveComponent("UBadge");
const UDropdownMenu = resolveComponent("UDropdownMenu");

const reloadKey = ref(0);
const dialog = ref(false);
// const items = ref<PostMetadataTable[]>();
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

const fetchData = () => {
  clearNuxtData("cache-table-key");
  refreshNuxtData("cache-table-key");
  reloadKey.value++;
};

const getDetail = async (id: number) => {
  selectedData.value = await $fetch<PostContentTable>(`/api/posts/${id}`, {
    method: "GET",
  });
};

const { data: items } = await useFetch<PostMetadataTable[]>("/api/posts", {
  method: "GET",
  key: "cache-table-key",
});
// items.value = data.value;
</script>

<template>
  <UButton
    icon="i-lucide-plus"
    label="Artikel"
    variant="soft"
    @click="
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
