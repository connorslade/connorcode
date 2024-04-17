import File from 'phosphor-svelte/lib/File';
import FileArchive from 'phosphor-svelte/lib/FileArchive';
import FileAudio from 'phosphor-svelte/lib/FileAudio';
import FileCode from 'phosphor-svelte/lib/FileCode';
import FileCsv from 'phosphor-svelte/lib/FileCsv';
import FileDoc from 'phosphor-svelte/lib/FileDoc';
import FileImage from 'phosphor-svelte/lib/FileImage';
import FilePdf from 'phosphor-svelte/lib/FilePdf';
import FileText from 'phosphor-svelte/lib/FileText';
import FileVideo from 'phosphor-svelte/lib/FileVideo';

const FILETYPE_MAP: { icon: typeof File; types: string[] }[] = [
	{ icon: FilePdf, types: ['pdf'] },
	{ icon: FileArchive, types: ['zip', 'tar', 'gz'] },
	{ icon: FileAudio, types: ['mp3', 'wav', 'flac'] },
	{ icon: FileCode, types: ['html', 'css', 'js', 'ts', 'wasm'] },
	{ icon: FileCsv, types: ['csv', 'tsv'] },
	{ icon: FileDoc, types: ['doc', 'docx', 'odf'] },
	{ icon: FileImage, types: ['png', 'jpg', 'jpeg', 'webp', 'gif', 'svg'] },
	{ icon: FileText, types: ['txt', 'md'] },
	{ icon: FileVideo, types: ['mp4', 'mkv'] }
];

export function from_ext(ext: string | undefined): typeof File {
	if (ext == undefined) return File;
	for (let filetype of FILETYPE_MAP) if (filetype.types.includes(ext)) return filetype.icon;
	return File;
}
