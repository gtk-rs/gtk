#import <AppKit/AppKit.h>

void macos_force_foreground_level() {
    [NSApp activateIgnoringOtherApps: YES];
}
