<script setup lang="ts">
import { APIResponse } from "@/types/response";

const props = defineProps<{
  response: APIResponse;
}>();
</script>
<template>
	<div class="container">
		<h1></h1>
        <pre>{{ props.response }}</pre>
	</div>
</template>

<style scoped>
.container {
	background-color: #393552;
}
</style>
