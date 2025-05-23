<template>
  <div
    class="flex flex-col p-6 transition-shadow bg-white rounded-lg shadow-md dark:bg-gray-800 hover:shadow-lg"
  >
    <div class="flex items-center justify-between mb-2">
      <div class="flex items-center space-x-2">
        <slot name="icon">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-8 h-8 text-blue-600 min-h-8 min-w-8"
            viewBox="0 0 48 48"
            xml:space="preserve"
            stroke="currentColor"
          >
            <path
              d="M16 3a13 13 0 1 0 13 13A13 13 0 0 0 16 3zm0 24a11 11 0 1 1 11-11 11 11 0 0 1-11 11z"
            />
            <path
              d="M16 8a1 1 0 0 0-1 1v6a1 1 0 0 0 .293.707l4 4a1 1 0 0 0 1.414-1.414L17 14.586V9a1 1 0 0 0-1-1z"
            />
          </svg>
        </slot>
        <h2 class="text-2xl font-semibold text-gray-800 dark:text-white">
          <slot name="title">{{ $t("admin.defaultTitle") }}</slot>
        </h2>
      </div>
    </div>
    <p class="flex-grow mb-4 text-xl text-gray-600 dark:text-gray-400">
      <slot name="description">{{ $t("admin.defaultDescription") }}</slot>
    </p>
    <div class="mt-auto space-y-4">
      <slot name="links">
        <DashboardLink
          to="/admin/default"
          type="secondary"
          :label="$t('admin.viewDefault')"
        />
        <DashboardLink
          to="/admin/default/create"
          type="default"
          :label="$t('admin.createDefault')"
        />
      </slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "AdminCard",
};
</script>
