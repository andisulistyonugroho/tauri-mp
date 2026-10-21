<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const open = ref(false);

function getItems(state: "collapsed" | "expanded") {
  return [
    {
      label: "Artikel",
      icon: "i-lucide-inbox",
      badge: "4",
    },
    {
      label: "Settings",
      icon: "i-lucide-settings",
      defaultOpen: false,
      children:
        state === "expanded"
          ? [
              {
                label: "General",
                icon: "i-lucide-house",
              },
              {
                label: "Team",
                icon: "i-lucide-users",
              },
              {
                label: "Billing",
                icon: "i-lucide-credit-card",
              },
            ]
          : [],
    },
  ] satisfies NavigationMenuItem[];
}
</script>

<template>
  <div class="flex flex-1">
    <USidebar
      v-model:open="open"
      collapsible="icon"
      rail
      :ui="{
        container: 'h-full',
        inner: 'bg-elevated/25 divide-transparent',
        body: 'py-0',
      }"
    >
      <template #header>
        <UButton
          variant="ghost"
          color="neutral"
          square
          class="w-full data-[state=open]:bg-elevated overflow-hidden"
          :ui="{
            trailingIcon: 'text-dimmed ms-auto',
          }"
          >MP</UButton
        >
      </template>

      <template #default="{ state }">
        <UNavigationMenu
          :key="state"
          :items="getItems(state)"
          orientation="vertical"
          :ui="{ link: 'p-1.5 overflow-hidden' }"
        />
      </template>
    </USidebar>

    <div class="flex-1 flex flex-col">
      <div
        class="h-(--ui-header-height) shrink-0 flex justify-between items-center px-4 border-b border-default"
      >
        <UButton
          icon="i-lucide-panel-left"
          color="neutral"
          variant="ghost"
          aria-label="Toggle sidebar"
          @click="open = !open"
        />
        <UButton color="error" icon="i-lucide-cloud-upload" variant="soft"
          >Upload</UButton
        >
      </div>

      <div class="flex-1 p-4">
        <UMain>
          <slot />
        </UMain>
      </div>
    </div>
  </div>
</template>
