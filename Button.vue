<template>
    <button
      :class="[
        'w-full block text-center py-2 rounded-md transition-colors',
        typeClasses,
        hoverClasses,
      ]"
      @click="handleClick"
    >
      <slot>{{ label }}</slot>
    </button>
  </template>
  
  <script setup>
  const props = defineProps({
    label: {
      type: String,
      default: "",
    },
    type: {
      type: String,
      default: "primary",
      validator: (value) =>
        ["primary", "secondary", "system", "roles", "users", "products", "logs"].includes(
          value
        ),
    },
  });
  
  const typeClasses = computed(() => {
    switch (props.type) {
      case "roles":
        return "text-xs bg-indigo-600 text-white hover:bg-indigo-700";
      case "users":
        return "text-xs bg-green-600 text-white hover:bg-green-700";
      case "products":
        return "text-xs bg-purple-600 text-white hover:bg-purple-700";
      case "logs":
        return "text-xs bg-yellow-600 text-white hover:bg-yellow-700";
      case "system":
        return "text-xs bg-red-600 text-white hover:bg-red-700";
      case "primary":
        return "text-xs bg-blue-600 text-white hover:bg-blue-700";
      case "secondary":
        return "text-xs border border-current text-current hover:bg-gray-600";
      default:
        return "text-xs bg-blue-600 text-white hover:bg-blue-700";
    }
  });
  
  // Add hover classes if necessary
  const hoverClasses = computed(() => "");
  </script>
  