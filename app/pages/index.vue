<script setup lang="ts">
import { h, resolveComponent } from "vue";
import type { TableColumn } from "@nuxt/ui";

const UBadge = resolveComponent("UBadge");

type Posts = {
  id: string;
  title: string;
  slug: string;
  category: string;
  published_at: string;
  tags: [];
};
const dialog = ref(false);
const items = ref<Posts[]>();

const columns: TableColumn<Posts>[] = [
  {
    accessorKey: "id",
    header: "#",
    cell: ({ row }) => `#${row.getValue("id")}`,
  },
  {
    accessorKey: "date",
    header: "Date",
    cell: ({ row }) => {
      return new Date(row.getValue("date")).toLocaleString("en-US", {
        day: "numeric",
        month: "short",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });
    },
  },
  {
    accessorKey: "status",
    header: "Status",
    cell: ({ row }) => {
      const color = {
        paid: "success" as const,
        failed: "error" as const,
        refunded: "neutral" as const,
      }[row.getValue("status") as string];

      return h(UBadge, { class: "capitalize", variant: "subtle", color }, () =>
        row.getValue("status"),
      );
    },
  },
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "amount",
    header: "Amount",
    meta: {
      class: {
        th: "text-right",
        td: "text-right font-medium",
      },
    },
    cell: ({ row }) => {
      const amount = Number.parseFloat(row.getValue("amount"));
      return new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "EUR",
      }).format(amount);
    },
  },
];

const fetchData = async () => {
  const { data } = await useFetch<Posts[]>("/api/posts", {
    method: "GET",
  });
  console.log("data:", data);
};

fetchData();
</script>

<template>
  <UButton
    icon="i-lucide-plus"
    label="Artikel"
    variant="soft"
    @click="dialog = true"
  />
  <UTable
    loading
    loading-color="primary"
    loading-animation="carousel"
    sticky
    :data="items"
    :columns="columns"
    class="flex-1"
  />

  <Formarticle v-model="dialog" />
</template>
