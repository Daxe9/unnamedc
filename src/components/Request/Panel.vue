<script setup lang="ts">
import { reactive } from "vue";

const previewSettings: Record<string, string> = reactive({});

const modes: any = reactive([
	{
		name: "HTTP REQUEST",
		value: ["GET", "POST" /*, "PUT", "DELETE", "PATCH"*/],
	},
	{
		name: "REQUEST BODY",
		value: ["JSON", "FORM", "RAW_TEXT"],
	},
]);

modes.forEach((mode: any) => {
	mode.visible = false;
});

function getValueOfSetting(e: any) {
	// parentElement
	// previousSibiling
	const setting = {
		value: e.target.textContent,
		name: e.target.parentElement.previousElementSibling.textContent,
	};

	previewSettings[setting.name] = setting.value;
}

function toggleList(name: string) {
	const mode = modes.find((mode: any) => mode.name === name);
	mode.visible = !mode.visible;
}
</script>

<template>
	<div class="request-panel">
		<div class="preview">
			<div>
				<h2 class="preview-title">Chose settings</h2>
			</div>
			<hr />
			<div class="settings-preview">
				<div
					v-for="(property, k) in Object.entries(previewSettings)"
					:key="k"
				>
					<div class="preview-single-setting">
						<span class="bullet-indicator">>></span>
						<span>{{ property[0] }}</span>
						<span class="cool-color-text">{{ property[1] }}</span>
					</div>
				</div>
			</div>
		</div>

		<div class="settings-select">
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
.preview-title {
	text-align: center;
}

.preview {
	border-top-left-radius: 0.3em;
	border-top-right-radius: 0.3em;
}

.settings-select {
	border-bottom-left-radius: 0.3em;
	border-bottom-right-radius: 0.3em;
}

.request-panel * {
	margin-left: 0.4em;
	margin-right: 0.4em;
}

.preview {
	margin: 1em 0.4em;
	padding: 0.4em 0;
	background-color: var(--dark-color);
}

.preview h2 {
	margin: 0;
	padding: 0;
}

.settings-select {
	background-color: var(--dark-color);
}

.setting-title {
	cursor: pointer;
}

.list-item {
	list-style-type: none;
	cursor: pointer;
}

.preview-single-setting {
	display: flex;
	flex-direction: row;
}

.bullet-indicator {
	font-weight: bold;
	font-size: 1.2em;
}
</style>
