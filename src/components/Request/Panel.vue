<script setup lang="ts">
import { reactive } from "vue";

// display preview of selected settings
const previewSettings: Record<string, string> = reactive({});

// available settings
const modes: any = reactive([
	{
		name: "METHOD",
		value: ["GET", "POST" /*, "PUT", "DELETE", "PATCH"*/],
	},
	{
		name: "BODY",
		value: ["JSON", "FORM", "RAW_TEXT"],
	},
]);


defineExpose({
    state: previewSettings,
    modes: modes
})

// make settings invisible on mount
modes.forEach((mode: any) => {
	mode.visible = false;
});

// TODO: remove a setting
// add settings to preview
function getValueOfSetting(e: any) {
    // get value of the element and the name of the setting from h3 element
	const setting = {
		value: e.target.textContent,
		name: e.target.parentElement.previousElementSibling.textContent,
	};

	previewSettings[setting.name.toLowerCase()] = setting.value;
}

// toggle visibility of settings
function toggleList(name: string) {
    // toggle the visibility of the list
	const mode = modes.find((mode: any) => mode.name === name);
	mode.visible = !mode.visible;
}
</script>

<template>
	<div class="request-panel">
		<div class="rounded-tl-lg rounded-tr-lg pt-1 mt-2 mb-3 preview">
			<div>
				<h2 class="text-center p-0 m-0 text-lg">Selected settings</h2>
			</div>
			<hr />
			<div class="settings-preview">
				<div
					v-for="(property, k) in Object.entries(previewSettings)"
					:key="k"
				>
					<div class="flex flex-row">
						<span class="bullet-indicator">>></span>
						<span>{{ property[0].toUpperCase() }}</span>
						<span class="cool-color-text">{{ property[1] }}</span>
					</div>
				</div>
			</div>
		</div>

		<div class="rounded-bl-lg rounded-br-lg py-2 settings-select">
			<div v-for="(mode, i) in modes" :key="i" class="prevent-select">
				<h3
					class="cool-color-text setting-title"
					@click="() => toggleList(mode.name)"
				>
					{{ mode.name }}
				</h3>
				<ul v-show="mode.visible">
					<li
						class="list-item"
						v-for="(type, j) in mode.value"
						:key="i + j"
						@click="getValueOfSetting"
					>
						{{ type }}
					</li>
				</ul>
			</div>
		</div>
	</div>
</template>

<style>
.settings-select {
	background-color: var(--dark-color);
}

.request-panel * {
	margin-left: 0.4em;
	margin-right: 0.4em;
}

.preview {
	background-color: var(--dark-color);
}

.setting-title {
	cursor: pointer;
}

.list-item {
	list-style-type: none;
	cursor: pointer;
}

.bullet-indicator {
	font-weight: bold;
}
</style>
