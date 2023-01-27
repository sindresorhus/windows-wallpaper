#include <comdef.h>
#include <wininet.h>
#include <ShlObj.h>
#include <fcntl.h>
#include <io.h>

void GetWallpaper(wchar_t *szWallpaper) {
	CoInitialize(NULL);   
	IActiveDesktop *pDesk;
	HRESULT hr = CoCreateInstance(CLSID_ActiveDesktop, NULL, CLSCTX_INPROC_SERVER, IID_IActiveDesktop, (void**)&pDesk);

	if (FAILED(hr)) {
		_com_error err(hr);
		fputs("Failed to get the desktop wallpaper", stderr);
		pDesk->Release();
		CoFreeUnusedLibraries();
		CoUninitialize();
	}

	pDesk->GetWallpaper(&szWallpaper[0], MAX_PATH, 0);
	pDesk->Release();
	CoFreeUnusedLibraries();
	CoUninitialize();
}

int SetWallpaper(LPWSTR fullPath, int wpStyle = WPSTYLE_SPAN) {
	CoInitialize(NULL);   
	IActiveDesktop *pDesk;
	HRESULT hr = CoCreateInstance(CLSID_ActiveDesktop, NULL, CLSCTX_INPROC_SERVER, IID_IActiveDesktop, (void**)&pDesk);

	if (FAILED(hr)) {
		_com_error err(hr);
		fputs("Failed to set the desktop wallpaper", stderr);
		pDesk->Release();
		CoFreeUnusedLibraries();
		CoUninitialize();
		return 1;
	}

	pDesk->SetWallpaper(fullPath, 0);
	WALLPAPEROPT wpOptions;
	wpOptions.dwSize = sizeof(WALLPAPEROPT);
	wpOptions.dwStyle = wpStyle;
	pDesk->SetWallpaperOptions(&wpOptions, 0);
	pDesk->ApplyChanges(AD_APPLY_ALL | AD_APPLY_FORCE | AD_APPLY_BUFFERED_REFRESH);
	pDesk->Release();
	CoFreeUnusedLibraries();
	CoUninitialize();
	return 0;
}

int wmain(int argc, wchar_t **argv) {

	if (argc <= 1) {
		wchar_t imagePath[MAX_PATH];
		GetWallpaper(imagePath);
		wprintf(L"%ls\n", imagePath);
	}

	if (wcscmp(argv[1], L"--version") == 0) {
		puts("1.1.2");
		return 0;
	}

	if (wcscmp(argv[1], L"--help") == 0) {
		puts("\n  Manage the desktop wallpaper\n\n  Usage: wallpaper [file]\n  Usage (scale): wallpaper [file] --scale [center | stretch | tile |  span | max | crop-to-fit | keep-aspect-ratio]\n\n  Created by Sindre Sorhus");
		return 0;
	}

	wchar_t fullPath[MAX_PATH];

	if (!_wfullpath(fullPath, argv[1], MAX_PATH)) {
		fputs("Invalid path", stderr);
		return 1;
	}

	if (argc <= 2) {
		SetWallpaper(fullPath);
	}

	/* There's some weirdness that occurs with wxscmp() when it is passed an empty argv:
		- The program will refuse to continue execution past the point of comparison.
			|_ 'else' conditions will not run despite the comparison not returning '0'

		This should not be problem for now due to the way the function was implemented.
		But keep this in mind for when we wish to extend functionality.

		Maybe it has something to do with the lack of null-terminators?
	*/
	if (wcscmp(argv[2], L"--scale") == 0) {
		if (wcscmp(argv[3], L"center") == 0) {
			SetWallpaper(fullPath, WPSTYLE_CENTER);
		} else if (wcscmp(argv[3], L"tile") == 0) {
			SetWallpaper(fullPath, WPSTYLE_TILE);
		} else if (wcscmp(argv[3], L"stretch") == 0) {
			SetWallpaper(fullPath, WPSTYLE_STRETCH);
		} else if (wcscmp(argv[3], L"keep-aspect-ratio") == 0) {
			SetWallpaper(fullPath, WPSTYLE_KEEPASPECT);
		} else if (wcscmp(argv[3], L"crop-to-fit") == 0) {
			SetWallpaper(fullPath, WPSTYLE_CROPTOFIT);
		} else if (wcscmp(argv[3], L"span") == 0) {
			SetWallpaper(fullPath, WPSTYLE_SPAN);
		} else if (wcscmp(argv[3], L"max") == 0) {
			SetWallpaper(fullPath, WPSTYLE_MAX);
		} else {
			SetWallpaper(fullPath);
		}
	}

	return 0;
}
