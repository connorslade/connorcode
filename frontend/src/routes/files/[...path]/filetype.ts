import {
	FilePdf,
	FileArchive,
	FileAudio,
	FileCode,
	FileCsv,
	FileDoc,
	FileImage,
	FileJpg,
	FileIni,
	FileText,
	FileVideo,
	FileZip,
	File
} from 'phosphor-svelte';

const FILETYPE_MAP: { icon: any; types: string[] }[] = [
	{ icon: FilePdf, types: ['pdf'] },
	{ icon: FileArchive, types: ['zip', 'tar', 'gz'] },
	{ icon: FileAudio, types: ['mp3', 'wav', 'flac'] },
	{ icon: FileCode, types: ['html', 'css', 'js', 'ts', 'wasm'] }
];

export function from_ext(ext: string | undefined): any {
	if (ext == undefined) return File;
	for (let filetype of FILETYPE_MAP) {
		if (filetype.types.includes(ext)) {
			return filetype.icon;
		}
	}
	return File;
}
