<script lang="ts">
	import Dropdown from '$lib/form-inputs/Dropdown.svelte';
	import FilePickerInput from '$lib/form-inputs/FilePickerInput.svelte';
	import ImagePickerInput from '$lib/form-inputs/ImagePickerInput.svelte';
	import TextInput from '$lib/form-inputs/TextInput.svelte';
	import ToggleSwitch from '$lib/form-inputs/ToggleSwitch.svelte';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';

	type DateFormat = 'YYYY-MM-DD' | 'DD/MM/YYYY' | 'MM/DD/YYYY' | 'DD MMM YYYY';

	type SettingsForm = {
		username: string;
		storageDirectory: string;
		dateFormat: DateFormat;
		profilePicture: string;
		deleteFromDeviceAfterStorageUpload: boolean;
	};

	type SettingsChanges = Partial<SettingsForm>;

	const dateFormatOptions: DateFormat[] = [
		'YYYY-MM-DD',
		'DD/MM/YYYY',
		'MM/DD/YYYY',
		'DD MMM YYYY'
	];

	let settings = $state<SettingsForm>({
		username: '',
		storageDirectory: '',
		dateFormat: 'YYYY-MM-DD',
		profilePicture: '',
		deleteFromDeviceAfterStorageUpload: false
	});

	let originalSettings = $state<SettingsForm | null>(null);
	let isLoading = $state(true);
	let isSaving = $state(false);

	let settingsChanges = $derived(getSettingsChanges(settings, originalSettings));
	let hasChanges = $derived(Object.keys(settingsChanges).length > 0);
	let profilePicturePreviewSrc = $derived(getProfilePicturePreviewSrc(settings.profilePicture));

	onMount(async () => {
		const fetchedSettings = await fetchCurrentSettings();

		settings = { ...fetchedSettings };
		originalSettings = { ...fetchedSettings };
		isLoading = false;
	});

	async function fetchCurrentSettings(): Promise<SettingsForm> {
		// Replace with: return await invoke<SettingsForm>('get_settings');
		return {
			username: 'Local user',
			storageDirectory: '/Users/local/Pictures/Frameport',
			dateFormat: 'YYYY-MM-DD',
			profilePicture: '/avatar-placeholder.svg',
			deleteFromDeviceAfterStorageUpload: false
		};
	}

	async function selectStorageDirectory() {
		const selectedDirectory = await open({
			multiple: false,
			directory: true
		});

		if (typeof selectedDirectory === 'string') {
			settings.storageDirectory = selectedDirectory;
		}
	}

	async function selectProfilePicture() {
		const selectedImage = await open({
			multiple: false,
			directory: false,
			filters: [
				{
					name: 'Image',
					extensions: ['png', 'jpg', 'jpeg', 'webp']
				}
			]
		});

		if (typeof selectedImage === 'string') {
			settings.profilePicture = selectedImage;
		}
	}

	async function saveChanges() {
		if (!hasChanges) {
			return;
		}

		isSaving = true;
		await saveSettingsChanges(settingsChanges);
		originalSettings = { ...settings };
		isSaving = false;
	}

	async function saveSettingsChanges(changes: SettingsChanges) {
		// Replace with: await invoke('update_settings', { changes });
		await Promise.resolve(changes);
	}

	function cancelChanges() {
		if (!originalSettings) {
			return;
		}

		settings = { ...originalSettings };
	}

	function getSettingsChanges(
		currentSettings: SettingsForm,
		previousSettings: SettingsForm | null
	): SettingsChanges {
		if (!previousSettings) {
			return {};
		}

		const changes: SettingsChanges = {};

		if (currentSettings.username !== previousSettings.username) {
			changes.username = currentSettings.username;
		}

		if (currentSettings.storageDirectory !== previousSettings.storageDirectory) {
			changes.storageDirectory = currentSettings.storageDirectory;
		}

		if (currentSettings.dateFormat !== previousSettings.dateFormat) {
			changes.dateFormat = currentSettings.dateFormat;
		}

		if (currentSettings.profilePicture !== previousSettings.profilePicture) {
			changes.profilePicture = currentSettings.profilePicture;
		}

		if (
			currentSettings.deleteFromDeviceAfterStorageUpload !==
			previousSettings.deleteFromDeviceAfterStorageUpload
		) {
			changes.deleteFromDeviceAfterStorageUpload =
				currentSettings.deleteFromDeviceAfterStorageUpload;
		}

		return changes;
	}

	function getProfilePicturePreviewSrc(profilePicture: string) {
		if (
			!profilePicture ||
			profilePicture.startsWith('/avatar-placeholder.svg') ||
			profilePicture.startsWith('asset:') ||
			profilePicture.startsWith('http') ||
			profilePicture.startsWith('data:') ||
			profilePicture.startsWith('blob:')
		) {
			return profilePicture;
		}

		return convertFileSrc(profilePicture);
	}
</script>

<main class="relative h-full overflow-hidden">
	<div
		class="h-full overflow-y-auto rounded-3xl border border-white/10 bg-neutral-950/70 p-8 pb-36"
	>
		<div class="max-w-4xl">
			<div>
				<p class="text-sm font-medium text-slate-400">Frameport</p>
				<h1 class="mt-2 text-3xl font-semibold text-slate-50">Settings</h1>
			</div>

			<div class="mt-8 grid gap-5">
				<TextInput
					label="Username"
					description="Shown in your local profile."
					bind:value={settings.username}
					disabled={isLoading}
				/>

				<FilePickerInput
					label="Storage directory"
					description="Where imported media is stored."
					bind:value={settings.storageDirectory}
					disabled={isLoading}
					onSelect={selectStorageDirectory}
				/>

				<Dropdown
					label="Date format"
					description="Used across media metadata views."
					options={dateFormatOptions}
					defaultOptionIndex={0}
					bind:value={settings.dateFormat}
					disabled={isLoading}
				/>

				<ImagePickerInput
					label="Profile picture"
					description="Local image used for your account."
					bind:value={settings.profilePicture}
					previewSrc={profilePicturePreviewSrc}
					disabled={isLoading}
					onSelect={selectProfilePicture}
				/>

				<ToggleSwitch
					label="Delete from device after storage upload"
					description="Remove source files once Frameport stores a copy."
					bind:value={settings.deleteFromDeviceAfterStorageUpload}
					disabled={isLoading}
				/>
			</div>
		</div>
	</div>

	{#if hasChanges}
		<div
			class="absolute right-6 bottom-6 left-6 flex h-20 items-center justify-between rounded-2xl border border-white/10 bg-neutral-950/95 px-5 shadow-2xl shadow-black/40 backdrop-blur"
		>
			<p class="text-sm text-slate-300">You have unsaved changes.</p>
			<div class="flex gap-3">
				<button
					type="button"
					onclick={cancelChanges}
					disabled={isSaving}
					class="h-11 rounded-xl border border-white/10 bg-white/4 px-5 text-sm font-medium text-slate-100 transition hover:bg-white/8 focus:outline-none focus:ring-2 focus:ring-white/10 disabled:opacity-60"
				>
					Cancel
				</button>
				<button
					type="button"
					onclick={saveChanges}
					disabled={isSaving}
					class="h-11 rounded-xl bg-slate-100 px-5 text-sm font-semibold text-neutral-950 transition hover:bg-white focus:outline-none focus:ring-2 focus:ring-white/30 disabled:opacity-60"
				>
					{isSaving ? 'Saving...' : 'Save changes'}
				</button>
			</div>
		</div>
	{/if}
</main>
