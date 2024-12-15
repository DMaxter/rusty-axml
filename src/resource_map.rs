#![allow(dead_code)]

//! AXML resource maps

use crate::chunk_header::ChunkHeader;
use crate::xml_types::XmlTypes;

use std::io::{
    Error,
    Cursor,
};
use byteorder::{
    LittleEndian,
    ReadBytesExt
};

/* Header of a chunk representing a resrouce map.
 * TODO: documentation
 */
pub struct ResourceMap {
    /* Chunk header */
    header: ChunkHeader,

    /* Resrouces IDs */
    resources_id: Vec<u32>,
}

impl ResourceMap {

    pub fn from_buff(axml_buff: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        /* Go back 2 bytes, to account from the block type */
        let offset = axml_buff.position();
        axml_buff.set_position(offset - 2);

        /* Parse chunk header */
        let header = ChunkHeader::from_buff(axml_buff, XmlTypes::ResXmlResourceMapType)
                     .expect("Error: cannot get chunk header from string pool");

        /* Get resources IDs */
        let mut resources_id = Vec::new();
        let nb_resources = (header.size / 4) - 2;
        for _ in 0..nb_resources {
            let id = axml_buff.read_u32::<LittleEndian>().unwrap();
            resources_id.push(id);
        }

        Ok(ResourceMap {
            header,
            resources_id
        })
    }
}

fn get_resource_string(mut id: u32) -> Result<String, Error> {
    let attr_names = vec![
        "theme",
        "label",
        "icon",
        "name",
        "manageSpaceActivity",
        "allowClearUserData",
        "permission",
        "readPermission",
        "writePermission",
        "protectionLevel",
        "permissionGroup",
        "sharedUserId",
        "hasCode",
        "persistent",
        "enabled",
        "debuggable",
        "exported",
        "process",
        "taskAffinity",
        "multiprocess",
        "finishOnTaskLaunch",
        "clearTaskOnLaunch",
        "stateNotNeeded",
        "excludeFromRecents",
        "authorities",
        "syncable",
        "initOrder",
        "grantUriPermissions",
        "priority",
        "launchMode",
        "screenOrientation",
        "configChanges",
        "description",
        "targetPackage",
        "handleProfiling",
        "functionalTest",
        "value",
        "resource",
        "mimeType",
        "scheme",
        "host",
        "port",
        "path",
        "pathPrefix",
        "pathPattern",
        "action",
        "data",
        "targetClass",
        "colorForeground",
        "colorBackground",
        "backgroundDimAmount",
        "disabledAlpha",
        "textAppearance",
        "textAppearanceInverse",
        "textColorPrimary",
        "textColorPrimaryDisableOnly",
        "textColorSecondary",
        "textColorPrimaryInverse",
        "textColorSecondaryInverse",
        "textColorPrimaryNoDisable",
        "textColorSecondaryNoDisable",
        "textColorPrimaryInverseNoDisable",
        "textColorSecondaryInverseNoDisable",
        "textColorHintInverse",
        "textAppearanceLarge",
        "textAppearanceMedium",
        "textAppearanceSmall",
        "textAppearanceLargeInverse",
        "textAppearanceMediumInverse",
        "textAppearanceSmallInverse",
        "textCheckMark",
        "textCheckMarkInverse",
        "buttonStyle",
        "buttonStyleSmall",
        "buttonStyleInset",
        "buttonStyleToggle",
        "galleryItemBackground",
        "listPreferredItemHeight",
        "expandableListPreferredItemPaddingLeft",
        "expandableListPreferredChildPaddingLeft",
        "expandableListPreferredItemIndicatorLeft",
        "expandableListPreferredItemIndicatorRight",
        "expandableListPreferredChildIndicatorLeft",
        "expandableListPreferredChildIndicatorRight",
        "windowBackground",
        "windowFrame",
        "windowNoTitle",
        "windowIsFloating",
        "windowIsTranslucent",
        "windowContentOverlay",
        "windowTitleSize",
        "windowTitleStyle",
        "windowTitleBackgroundStyle",
        "alertDialogStyle",
        "panelBackground",
        "panelFullBackground",
        "panelColorForeground",
        "panelColorBackground",
        "panelTextAppearance",
        "scrollbarSize",
        "scrollbarThumbHorizontal",
        "scrollbarThumbVertical",
        "scrollbarTrackHorizontal",
        "scrollbarTrackVertical",
        "scrollbarAlwaysDrawHorizontalTrack",
        "scrollbarAlwaysDrawVerticalTrack",
        "absListViewStyle",
        "autoCompleteTextViewStyle",
        "checkboxStyle",
        "dropDownListViewStyle",
        "editTextStyle",
        "expandableListViewStyle",
        "galleryStyle",
        "gridViewStyle",
        "imageButtonStyle",
        "imageWellStyle",
        "listViewStyle",
        "listViewWhiteStyle",
        "popupWindowStyle",
        "progressBarStyle",
        "progressBarStyleHorizontal",
        "progressBarStyleSmall",
        "progressBarStyleLarge",
        "seekBarStyle",
        "ratingBarStyle",
        "ratingBarStyleSmall",
        "radioButtonStyle",
        "scrollbarStyle",
        "scrollViewStyle",
        "spinnerStyle",
        "starStyle",
        "tabWidgetStyle",
        "textViewStyle",
        "webViewStyle",
        "dropDownItemStyle",
        "spinnerDropDownItemStyle",
        "dropDownHintAppearance",
        "spinnerItemStyle",
        "mapViewStyle",
        "preferenceScreenStyle",
        "preferenceCategoryStyle",
        "preferenceInformationStyle",
        "preferenceStyle",
        "checkBoxPreferenceStyle",
        "yesNoPreferenceStyle",
        "dialogPreferenceStyle",
        "editTextPreferenceStyle",
        "ringtonePreferenceStyle",
        "preferenceLayoutChild",
        "textSize",
        "typeface",
        "textStyle",
        "textColor",
        "textColorHighlight",
        "textColorHint",
        "textColorLink",
        "state_focused",
        "state_window_focused",
        "state_enabled",
        "state_checkable",
        "state_checked",
        "state_selected",
        "state_active",
        "state_single",
        "state_first",
        "state_middle",
        "state_last",
        "state_pressed",
        "state_expanded",
        "state_empty",
        "state_above_anchor",
        "ellipsize",
        "x",
        "y",
        "windowAnimationStyle",
        "gravity",
        "autoLink",
        "linksClickable",
        "entries",
        "layout_gravity",
        "windowEnterAnimation",
        "windowExitAnimation",
        "windowShowAnimation",
        "windowHideAnimation",
        "activityOpenEnterAnimation",
        "activityOpenExitAnimation",
        "activityCloseEnterAnimation",
        "activityCloseExitAnimation",
        "taskOpenEnterAnimation",
        "taskOpenExitAnimation",
        "taskCloseEnterAnimation",
        "taskCloseExitAnimation",
        "taskToFrontEnterAnimation",
        "taskToFrontExitAnimation",
        "taskToBackEnterAnimation",
        "taskToBackExitAnimation",
        "orientation",
        "keycode",
        "fullDark",
        "topDark",
        "centerDark",
        "bottomDark",
        "fullBright",
        "topBright",
        "centerBright",
        "bottomBright",
        "bottomMedium",
        "centerMedium",
        "id",
        "tag",
        "scrollX",
        "scrollY",
        "background",
        "padding",
        "paddingLeft",
        "paddingTop",
        "paddingRight",
        "paddingBottom",
        "focusable",
        "focusableInTouchMode",
        "visibility",
        "fitsSystemWindows",
        "scrollbars",
        "fadingEdge",
        "fadingEdgeLength",
        "nextFocusLeft",
        "nextFocusRight",
        "nextFocusUp",
        "nextFocusDown",
        "clickable",
        "longClickable",
        "saveEnabled",
        "drawingCacheQuality",
        "duplicateParentState",
        "clipChildren",
        "clipToPadding",
        "layoutAnimation",
        "animationCache",
        "persistentDrawingCache",
        "alwaysDrawnWithCache",
        "addStatesFromChildren",
        "descendantFocusability",
        "layout",
        "inflatedId",
        "layout_width",
        "layout_height",
        "layout_margin",
        "layout_marginLeft",
        "layout_marginTop",
        "layout_marginRight",
        "layout_marginBottom",
        "listSelector",
        "drawSelectorOnTop",
        "stackFromBottom",
        "scrollingCache",
        "textFilterEnabled",
        "transcriptMode",
        "cacheColorHint",
        "dial",
        "hand_hour",
        "hand_minute",
        "format",
        "checked",
        "button",
        "checkMark",
        "foreground",
        "measureAllChildren",
        "groupIndicator",
        "childIndicator",
        "indicatorLeft",
        "indicatorRight",
        "childIndicatorLeft",
        "childIndicatorRight",
        "childDivider",
        "animationDuration",
        "spacing",
        "horizontalSpacing",
        "verticalSpacing",
        "stretchMode",
        "columnWidth",
        "numColumns",
        "src",
        "antialias",
        "filter",
        "dither",
        "scaleType",
        "adjustViewBounds",
        "maxWidth",
        "maxHeight",
        "tint",
        "baselineAlignBottom",
        "cropToPadding",
        "textOn",
        "textOff",
        "baselineAligned",
        "baselineAlignedChildIndex",
        "weightSum",
        "divider",
        "dividerHeight",
        "choiceMode",
        "itemTextAppearance",
        "horizontalDivider",
        "verticalDivider",
        "headerBackground",
        "itemBackground",
        "itemIconDisabledAlpha",
        "rowHeight",
        "maxRows",
        "maxItemsPerRow",
        "moreIcon",
        "max",
        "progress",
        "secondaryProgress",
        "indeterminate",
        "indeterminateOnly",
        "indeterminateDrawable",
        "progressDrawable",
        "indeterminateDuration",
        "indeterminateBehavior",
        "minWidth",
        "minHeight",
        "interpolator",
        "thumb",
        "thumbOffset",
        "numStars",
        "rating",
        "stepSize",
        "isIndicator",
        "checkedButton",
        "stretchColumns",
        "shrinkColumns",
        "collapseColumns",
        "layout_column",
        "layout_span",
        "bufferType",
        "text",
        "hint",
        "textScaleX",
        "cursorVisible",
        "maxLines",
        "lines",
        "height",
        "minLines",
        "maxEms",
        "ems",
        "width",
        "minEms",
        "scrollHorizontally",
        "password",
        "singleLine",
        "selectAllOnFocus",
        "includeFontPadding",
        "maxLength",
        "shadowColor",
        "shadowDx",
        "shadowDy",
        "shadowRadius",
        "numeric",
        "digits",
        "phoneNumber",
        "inputMethod",
        "capitalize",
        "autoText",
        "editable",
        "freezesText",
        "drawableTop",
        "drawableBottom",
        "drawableLeft",
        "drawableRight",
        "drawablePadding",
        "completionHint",
        "completionHintView",
        "completionThreshold",
        "dropDownSelector",
        "popupBackground",
        "inAnimation",
        "outAnimation",
        "flipInterval",
        "fillViewport",
        "prompt",
        "startYear",
        "endYear",
        "mode",
        "layout_x",
        "layout_y",
        "layout_weight",
        "layout_toLeftOf",
        "layout_toRightOf",
        "layout_above",
        "layout_below",
        "layout_alignBaseline",
        "layout_alignLeft",
        "layout_alignTop",
        "layout_alignRight",
        "layout_alignBottom",
        "layout_alignParentLeft",
        "layout_alignParentTop",
        "layout_alignParentRight",
        "layout_alignParentBottom",
        "layout_centerInParent",
        "layout_centerHorizontal",
        "layout_centerVertical",
        "layout_alignWithParentIfMissing",
        "layout_scale",
        "visible",
        "variablePadding",
        "constantSize",
        "oneshot",
        "duration",
        "drawable",
        "shape",
        "innerRadiusRatio",
        "thicknessRatio",
        "startColor",
        "endColor",
        "useLevel",
        "angle",
        "type",
        "centerX",
        "centerY",
        "gradientRadius",
        "color",
        "dashWidth",
        "dashGap",
        "radius",
        "topLeftRadius",
        "topRightRadius",
        "bottomLeftRadius",
        "bottomRightRadius",
        "left",
        "top",
        "right",
        "bottom",
        "minLevel",
        "maxLevel",
        "fromDegrees",
        "toDegrees",
        "pivotX",
        "pivotY",
        "insetLeft",
        "insetRight",
        "insetTop",
        "insetBottom",
        "shareInterpolator",
        "fillBefore",
        "fillAfter",
        "startOffset",
        "repeatCount",
        "repeatMode",
        "zAdjustment",
        "fromXScale",
        "toXScale",
        "fromYScale",
        "toYScale",
        "fromXDelta",
        "toXDelta",
        "fromYDelta",
        "toYDelta",
        "fromAlpha",
        "toAlpha",
        "delay",
        "animation",
        "animationOrder",
        "columnDelay",
        "rowDelay",
        "direction",
        "directionPriority",
        "factor",
        "cycles",
        "searchMode",
        "searchSuggestAuthority",
        "searchSuggestPath",
        "searchSuggestSelection",
        "searchSuggestIntentAction",
        "searchSuggestIntentData",
        "queryActionMsg",
        "suggestActionMsg",
        "suggestActionMsgColumn",
        "menuCategory",
        "orderInCategory",
        "checkableBehavior",
        "title",
        "titleCondensed",
        "alphabeticShortcut",
        "numericShortcut",
        "checkable",
        "selectable",
        "orderingFromXml",
        "key",
        "summary",
        "order",
        "widgetLayout",
        "dependency",
        "defaultValue",
        "shouldDisableView",
        "summaryOn",
        "summaryOff",
        "disableDependentsState",
        "dialogTitle",
        "dialogMessage",
        "dialogIcon",
        "positiveButtonText",
        "negativeButtonText",
        "dialogLayout",
        "entryValues",
        "ringtoneType",
        "showDefault",
        "showSilent",
        "scaleWidth",
        "scaleHeight",
        "scaleGravity",
        "ignoreGravity",
        "foregroundGravity",
        "tileMode",
        "targetActivity",
        "alwaysRetainTaskState",
        "allowTaskReparenting",
        "searchButtonText",
        "colorForegroundInverse",
        "textAppearanceButton",
        "listSeparatorTextViewStyle",
        "streamType",
        "clipOrientation",
        "centerColor",
        "minSdkVersion",
        "windowFullscreen",
        "unselectedAlpha",
        "progressBarStyleSmallTitle",
        "ratingBarStyleIndicator",
        "apiKey",
        "textColorTertiary",
        "textColorTertiaryInverse",
        "listDivider",
        "soundEffectsEnabled",
        "keepScreenOn",
        "lineSpacingExtra",
        "lineSpacingMultiplier",
        "listChoiceIndicatorSingle",
        "listChoiceIndicatorMultiple",
        "versionCode",
        "versionName",
        "marqueeRepeatLimit",
        "windowNoDisplay",
        "backgroundDimEnabled",
        "inputType",
        "isDefault",
        "windowDisablePreview",
        "privateImeOptions",
        "editorExtras",
        "settingsActivity",
        "fastScrollEnabled",
        "reqTouchScreen",
        "reqKeyboardType",
        "reqHardKeyboard",
        "reqNavigation",
        "windowSoftInputMode",
        "imeFullscreenBackground",
        "noHistory",
        "headerDividersEnabled",
        "footerDividersEnabled",
        "candidatesTextStyleSpans",
        "smoothScrollbar",
        "reqFiveWayNav",
        "keyBackground",
        "keyTextSize",
        "labelTextSize",
        "keyTextColor",
        "keyPreviewLayout",
        "keyPreviewOffset",
        "keyPreviewHeight",
        "verticalCorrection",
        "popupLayout",
        "state_long_pressable",
        "keyWidth",
        "keyHeight",
        "horizontalGap",
        "verticalGap",
        "rowEdgeFlags",
        "codes",
        "popupKeyboard",
        "popupCharacters",
        "keyEdgeFlags",
        "isModifier",
        "isSticky",
        "isRepeatable",
        "iconPreview",
        "keyOutputText",
        "keyLabel",
        "keyIcon",
        "keyboardMode",
        "isScrollContainer",
        "fillEnabled",
        "updatePeriodMillis",
        "initialLayout",
        "voiceSearchMode",
        "voiceLanguageModel",
        "voicePromptText",
        "voiceLanguage",
        "voiceMaxResults",
        "bottomOffset",
        "topOffset",
        "allowSingleTap",
        "handle",
        "content",
        "animateOnClick",
        "configure",
        "hapticFeedbackEnabled",
        "innerRadius",
        "thickness",
        "sharedUserLabel",
        "dropDownWidth",
        "dropDownAnchor",
        "imeOptions",
        "imeActionLabel",
        "imeActionId",
        "UNKNOWN",
        "imeExtractEnterAnimation",
        "imeExtractExitAnimation",
        "tension",
        "extraTension",
        "anyDensity",
        "searchSuggestThreshold",
        "includeInGlobalSearch",
        "onClick",
        "targetSdkVersion",
        "maxSdkVersion",
        "testOnly",
        "contentDescription",
        "gestureStrokeWidth",
        "gestureColor",
        "uncertainGestureColor",
        "fadeOffset",
        "fadeDuration",
        "gestureStrokeType",
        "gestureStrokeLengthThreshold",
        "gestureStrokeSquarenessThreshold",
        "gestureStrokeAngleThreshold",
        "eventsInterceptionEnabled",
        "fadeEnabled",
        "backupAgent",
        "allowBackup",
        "glEsVersion",
        "queryAfterZeroResults",
        "dropDownHeight",
        "smallScreens",
        "normalScreens",
        "largeScreens",
        "progressBarStyleInverse",
        "progressBarStyleSmallInverse",
        "progressBarStyleLargeInverse",
        "searchSettingsDescription",
        "textColorPrimaryInverseDisableOnly",
        "autoUrlDetect",
        "resizeable",
        "required",
        "accountType",
        "contentAuthority",
        "userVisible",
        "windowShowWallpaper",
        "wallpaperOpenEnterAnimation",
        "wallpaperOpenExitAnimation",
        "wallpaperCloseEnterAnimation",
        "wallpaperCloseExitAnimation",
        "wallpaperIntraOpenEnterAnimation",
        "wallpaperIntraOpenExitAnimation",
        "wallpaperIntraCloseEnterAnimation",
        "wallpaperIntraCloseExitAnimation",
        "supportsUploading",
        "killAfterRestore",
        "restoreNeedsApplication",
        "smallIcon",
        "accountPreferences",
        "textAppearanceSearchResultSubtitle",
        "textAppearanceSearchResultTitle",
        "summaryColumn",
        "detailColumn",
        "detailSocialSummary",
        "thumbnail",
        "detachWallpaper",
        "finishOnCloseSystemDialogs",
        "scrollbarFadeDuration",
        "scrollbarDefaultDelayBeforeFade",
        "fadeScrollbars",
        "colorBackgroundCacheHint",
        "dropDownHorizontalOffset",
        "dropDownVerticalOffset",
        "quickContactBadgeStyleWindowSmall",
        "quickContactBadgeStyleWindowMedium",
        "quickContactBadgeStyleWindowLarge",
        "quickContactBadgeStyleSmallWindowSmall",
        "quickContactBadgeStyleSmallWindowMedium",
        "quickContactBadgeStyleSmallWindowLarge",
        "author",
        "autoStart",
        "expandableListViewWhiteStyle",
        "installLocation",
        "vmSafeMode",
        "webTextViewStyle",
        "restoreAnyVersion",
        "tabStripLeft",
        "tabStripRight",
        "tabStripEnabled",
        "logo",
        "xlargeScreens",
        "immersive",
        "overScrollMode",
        "overScrollHeader",
        "overScrollFooter",
        "filterTouchesWhenObscured",
        "textSelectHandleLeft",
        "textSelectHandleRight",
        "textSelectHandle",
        "textSelectHandleWindowStyle",
        "popupAnimationStyle",
        "screenSize",
        "screenDensity",
        "allContactsName",
        "windowActionBar",
        "actionBarStyle",
        "navigationMode",
        "displayOptions",
        "subtitle",
        "customNavigationLayout",
        "hardwareAccelerated",
        "measureWithLargestChild",
        "animateFirstView",
        "dropDownSpinnerStyle",
        "actionDropDownStyle",
        "actionButtonStyle",
        "showAsAction",
        "previewImage",
        "actionModeBackground",
        "actionModeCloseDrawable",
        "windowActionModeOverlay",
        "valueFrom",
        "valueTo",
        "valueType",
        "propertyName",
        "ordering",
        "fragment",
        "windowActionBarOverlay",
        "fragmentOpenEnterAnimation",
        "fragmentOpenExitAnimation",
        "fragmentCloseEnterAnimation",
        "fragmentCloseExitAnimation",
        "fragmentFadeEnterAnimation",
        "fragmentFadeExitAnimation",
        "actionBarSize",
        "imeSubtypeLocale",
        "imeSubtypeMode",
        "imeSubtypeExtraValue",
        "splitMotionEvents",
        "listChoiceBackgroundIndicator",
        "spinnerMode",
        "animateLayoutChanges",
        "actionBarTabStyle",
        "actionBarTabBarStyle",
        "actionBarTabTextStyle",
        "actionOverflowButtonStyle",
        "actionModeCloseButtonStyle",
        "titleTextStyle",
        "subtitleTextStyle",
        "iconifiedByDefault",
        "actionLayout",
        "actionViewClass",
        "activatedBackgroundIndicator",
        "state_activated",
        "listPopupWindowStyle",
        "popupMenuStyle",
        "textAppearanceLargePopupMenu",
        "textAppearanceSmallPopupMenu",
        "breadCrumbTitle",
        "breadCrumbShortTitle",
        "listDividerAlertDialog",
        "textColorAlertDialogListItem",
        "loopViews",
        "dialogTheme",
        "alertDialogTheme",
        "dividerVertical",
        "homeAsUpIndicator",
        "enterFadeDuration",
        "exitFadeDuration",
        "selectableItemBackground",
        "autoAdvanceViewId",
        "useIntrinsicSizeAsMinimum",
        "actionModeCutDrawable",
        "actionModeCopyDrawable",
        "actionModePasteDrawable",
        "textEditPasteWindowLayout",
        "textEditNoPasteWindowLayout",
        "textIsSelectable",
        "windowEnableSplitTouch",
        "indeterminateProgressStyle",
        "progressBarPadding",
        "animationResolution",
        "state_accelerated",
        "baseline",
        "homeLayout",
        "opacity",
        "alpha",
        "transformPivotX",
        "transformPivotY",
        "translationX",
        "translationY",
        "scaleX",
        "scaleY",
        "rotation",
        "rotationX",
        "rotationY",
        "showDividers",
        "dividerPadding",
        "borderlessButtonStyle",
        "dividerHorizontal",
        "itemPadding",
        "buttonBarStyle",
        "buttonBarButtonStyle",
        "segmentedButtonStyle",
        "staticWallpaperPreview",
        "allowParallelSyncs",
        "isAlwaysSyncable",
        "verticalScrollbarPosition",
        "fastScrollAlwaysVisible",
        "fastScrollThumbDrawable",
        "fastScrollPreviewBackgroundLeft",
        "fastScrollPreviewBackgroundRight",
        "fastScrollTrackDrawable",
        "fastScrollOverlayPosition",
        "customTokens",
        "nextFocusForward",
        "firstDayOfWeek",
        "showWeekNumber",
        "minDate",
        "maxDate",
        "shownWeekCount",
        "selectedWeekBackgroundColor",
        "focusedMonthDateColor",
        "unfocusedMonthDateColor",
        "weekNumberColor",
        "weekSeparatorLineColor",
        "selectedDateVerticalBar",
        "weekDayTextAppearance",
        "dateTextAppearance",
        "UNKNOWN",
        "spinnersShown",
        "calendarViewShown",
        "state_multiline",
        "detailsElementBackground",
        "textColorHighlightInverse",
        "textColorLinkInverse",
        "editTextColor",
        "editTextBackground",
        "horizontalScrollViewStyle",
        "layerType",
        "alertDialogIcon",
        "windowMinWidthMajor",
        "windowMinWidthMinor",
        "queryHint",
        "fastScrollTextColor",
        "largeHeap",
        "windowCloseOnTouchOutside",
        "datePickerStyle",
        "calendarViewStyle",
        "textEditSidePasteWindowLayout",
        "textEditSideNoPasteWindowLayout",
        "actionMenuTextAppearance",
        "actionMenuTextColor",
        "textCursorDrawable",
        "resizeMode",
        "requiresSmallestWidthDp",
        "compatibleWidthLimitDp",
        "largestWidthLimitDp",
        "state_hovered",
        "state_drag_can_accept",
        "state_drag_hovered",
        "stopWithTask",
        "switchTextOn",
        "switchTextOff",
        "switchPreferenceStyle",
        "switchTextAppearance",
        "track",
        "switchMinWidth",
        "switchPadding",
        "thumbTextPadding",
        "textSuggestionsWindowStyle",
        "textEditSuggestionItemLayout",
        "rowCount",
        "rowOrderPreserved",
        "columnCount",
        "columnOrderPreserved",
        "useDefaultMargins",
        "alignmentMode",
        "layout_row",
        "layout_rowSpan",
        "layout_columnSpan",
        "actionModeSelectAllDrawable",
        "isAuxiliary",
        "accessibilityEventTypes",
        "packageNames",
        "accessibilityFeedbackType",
        "notificationTimeout",
        "accessibilityFlags",
        "canRetrieveWindowContent",
        "listPreferredItemHeightLarge",
        "listPreferredItemHeightSmall",
        "actionBarSplitStyle",
        "actionProviderClass",
        "backgroundStacked",
        "backgroundSplit",
        "textAllCaps",
        "colorPressedHighlight",
        "colorLongPressedHighlight",
        "colorFocusedHighlight",
        "colorActivatedHighlight",
        "colorMultiSelectHighlight",
        "drawableStart",
        "drawableEnd",
        "actionModeStyle",
        "minResizeWidth",
        "minResizeHeight",
        "actionBarWidgetTheme",
        "uiOptions",
        "subtypeLocale",
        "subtypeExtraValue",
        "actionBarDivider",
        "actionBarItemBackground",
        "actionModeSplitBackground",
        "textAppearanceListItem",
        "textAppearanceListItemSmall",
        "targetDescriptions",
        "directionDescriptions",
        "overridesImplicitlyEnabledSubtype",
        "listPreferredItemPaddingLeft",
        "listPreferredItemPaddingRight",
        "requiresFadingEdge",
        "publicKey",
        "parentActivityName",
        "UNKNOWN",
        "isolatedProcess",
        "importantForAccessibility",
        "keyboardLayout",
        "fontFamily",
        "mediaRouteButtonStyle",
        "mediaRouteTypes",
        "supportsRtl",
        "textDirection",
        "textAlignment",
        "layoutDirection",
        "paddingStart",
        "paddingEnd",
        "layout_marginStart",
        "layout_marginEnd",
        "layout_toStartOf",
        "layout_toEndOf",
        "layout_alignStart",
        "layout_alignEnd",
        "layout_alignParentStart",
        "layout_alignParentEnd",
        "listPreferredItemPaddingStart",
        "listPreferredItemPaddingEnd",
        "singleUser",
        "presentationTheme",
        "subtypeId",
        "initialKeyguardLayout",
        "UNKNOWN",
        "widgetCategory",
        "permissionGroupFlags",
        "labelFor",
        "permissionFlags",
        "checkedTextViewStyle",
        "showOnLockScreen",
        "format12Hour",
        "format24Hour",
        "timeZone",
        "mipMap",
        "mirrorForRtl",
        "windowOverscan",
        "requiredForAllUsers",
        "indicatorStart",
        "indicatorEnd",
        "childIndicatorStart",
        "childIndicatorEnd",
        "restrictedAccountType",
        "requiredAccountType",
        "canRequestTouchExplorationMode",
        "canRequestEnhancedWebAccessibility",
        "canRequestFilterKeyEvents",
        "layoutMode",
        "keySet",
        "targetId",
        "fromScene",
        "toScene",
        "transition",
        "transitionOrdering",
        "fadingMode",
        "startDelay",
        "ssp",
        "sspPrefix",
        "sspPattern",
        "addPrintersActivity",
        "vendor",
        "category",
        "isAsciiCapable",
        "autoMirrored",
        "supportsSwitchingToNextInputMethod",
        "requireDeviceUnlock",
        "apduServiceBanner",
        "accessibilityLiveRegion",
        "windowTranslucentStatus",
        "windowTranslucentNavigation",
        "advancedPrintOptionsActivity",
        "banner",
        "windowSwipeToDismiss",
        "isGame",
        "allowEmbedded",
        "setupActivity",
        "fastScrollStyle",
        "windowContentTransitions",
        "windowContentTransitionManager",
        "translationZ",
        "tintMode",
        "controlX1",
        "controlY1",
        "controlX2",
        "controlY2",
        "transitionName",
        "transitionGroup",
        "viewportWidth",
        "viewportHeight",
        "fillColor",
        "pathData",
        "strokeColor",
        "strokeWidth",
        "trimPathStart",
        "trimPathEnd",
        "trimPathOffset",
        "strokeLineCap",
        "strokeLineJoin",
        "strokeMiterLimit",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "colorControlNormal",
        "colorControlActivated",
        "colorButtonNormal",
        "colorControlHighlight",
        "persistableMode",
        "titleTextAppearance",
        "subtitleTextAppearance",
        "slideEdge",
        "actionBarTheme",
        "textAppearanceListItemSecondary",
        "colorPrimary",
        "colorPrimaryDark",
        "colorAccent",
        "nestedScrollingEnabled",
        "windowEnterTransition",
        "windowExitTransition",
        "windowSharedElementEnterTransition",
        "windowSharedElementExitTransition",
        "windowAllowReturnTransitionOverlap",
        "windowAllowEnterTransitionOverlap",
        "sessionService",
        "stackViewStyle",
        "switchStyle",
        "elevation",
        "excludeId",
        "excludeClass",
        "hideOnContentScroll",
        "actionOverflowMenuStyle",
        "documentLaunchMode",
        "maxRecents",
        "autoRemoveFromRecents",
        "stateListAnimator",
        "toId",
        "fromId",
        "reversible",
        "splitTrack",
        "targetName",
        "excludeName",
        "matchOrder",
        "windowDrawsSystemBarBackgrounds",
        "statusBarColor",
        "navigationBarColor",
        "contentInsetStart",
        "contentInsetEnd",
        "contentInsetLeft",
        "contentInsetRight",
        "paddingMode",
        "layout_rowWeight",
        "layout_columnWeight",
        "translateX",
        "translateY",
        "selectableItemBackgroundBorderless",
        "elegantTextHeight",
        "UNKNOWN",
        "UNKNOWN",
        "UNKNOWN",
        "windowTransitionBackgroundFadeDuration",
        "overlapAnchor",
        "progressTint",
        "progressTintMode",
        "progressBackgroundTint",
        "progressBackgroundTintMode",
        "secondaryProgressTint",
        "secondaryProgressTintMode",
        "indeterminateTint",
        "indeterminateTintMode",
        "backgroundTint",
        "backgroundTintMode",
        "foregroundTint",
        "foregroundTintMode",
        "buttonTint",
        "buttonTintMode",
        "thumbTint",
        "thumbTintMode",
        "fullBackupOnly",
        "propertyXName",
        "propertyYName",
        "relinquishTaskIdentity",
        "tileModeX",
        "tileModeY",
        "actionModeShareDrawable",
        "actionModeFindDrawable",
        "actionModeWebSearchDrawable",
        "transitionVisibilityMode",
        "minimumHorizontalAngle",
        "minimumVerticalAngle",
        "maximumAngle",
        "searchViewStyle",
        "closeIcon",
        "goIcon",
        "searchIcon",
        "voiceIcon",
        "commitIcon",
        "suggestionRowLayout",
        "queryBackground",
        "submitBackground",
        "buttonBarPositiveButtonStyle",
        "buttonBarNeutralButtonStyle",
        "buttonBarNegativeButtonStyle",
        "popupElevation",
        "actionBarPopupTheme",
        "multiArch",
        "touchscreenBlocksFocus",
        "windowElevation",
        "launchTaskBehindTargetAnimation",
        "launchTaskBehindSourceAnimation",
        "restrictionType",
        "dayOfWeekBackground",
        "dayOfWeekTextAppearance",
        "headerMonthTextAppearance",
        "headerDayOfMonthTextAppearance",
        "headerYearTextAppearance",
        "yearListItemTextAppearance",
        "yearListSelectorColor",
        "calendarTextColor",
        "recognitionService",
        "timePickerStyle",
        "timePickerDialogTheme",
        "headerTimeTextAppearance",
        "headerAmPmTextAppearance",
        "numbersTextColor",
        "numbersBackgroundColor",
        "numbersSelectorColor",
        "amPmTextColor",
        "amPmBackgroundColor",
        "UNKNOWN",
        "checkMarkTint",
        "checkMarkTintMode",
        "popupTheme",
        "toolbarStyle",
        "windowClipToOutline",
        "datePickerDialogTheme",
        "showText",
        "windowReturnTransition",
        "windowReenterTransition",
        "windowSharedElementReturnTransition",
        "windowSharedElementReenterTransition",
        "resumeWhilePausing",
        "datePickerMode",
        "timePickerMode",
        "inset",
        "letterSpacing",
        "fontFeatureSettings",
        "outlineProvider",
        "contentAgeHint",
        "country",
        "windowSharedElementsUseOverlay",
        "reparent",
        "reparentWithOverlay",
        "ambientShadowAlpha",
        "spotShadowAlpha",
        "navigationIcon",
        "navigationContentDescription",
        "fragmentExitTransition",
        "fragmentEnterTransition",
        "fragmentSharedElementEnterTransition",
        "fragmentReturnTransition",
        "fragmentSharedElementReturnTransition",
        "fragmentReenterTransition",
        "fragmentAllowEnterTransitionOverlap",
        "fragmentAllowReturnTransitionOverlap",
        "patternPathData",
        "strokeAlpha",
        "fillAlpha",
        "windowActivityTransitions",
        "colorEdgeEffect",
        "resizeClip",
        "collapseContentDescription",
        "accessibilityTraversalBefore",
        "accessibilityTraversalAfter",
        "dialogPreferredPadding",
        "searchHintIcon",
        "revisionCode",
        "drawableTint",
        "drawableTintMode",
        "fraction",
        "trackTint",
        "trackTintMode",
        "start",
        "end",
        "breakStrategy",
        "hyphenationFrequency",
        "allowUndo",
        "windowLightStatusBar",
        "numbersInnerTextColor",
        "colorBackgroundFloating",
        "titleTextColor",
        "subtitleTextColor",
        "thumbPosition",
        "scrollIndicators",
        "contextClickable",
        "fingerprintAuthDrawable",
        "logoDescription",
        "extractNativeLibs",
        "fullBackupContent",
        "usesCleartextTraffic",
        "lockTaskMode",
        "autoVerify",
        "showForAllUsers",
        "supportsAssist",
        "supportsLaunchVoiceAssistFromKeyguard",
        "listMenuViewStyle",
        "subMenuArrow",
        "defaultWidth",
        "defaultHeight",
        "resizeableActivity",
        "supportsPictureInPicture",
        "titleMargin",
        "titleMarginStart",
        "titleMarginEnd",
        "titleMarginTop",
        "titleMarginBottom",
        "maxButtonHeight",
        "buttonGravity",
        "collapseIcon",
        "level",
        "contextPopupMenuStyle",
        "textAppearancePopupMenuHeader",
        "windowBackgroundFallback",
        "defaultToDeviceProtectedStorage",
        "directBootAware",
        "preferenceFragmentStyle",
        "canControlMagnification",
        "languageTag",
        "pointerIcon",
        "tickMark",
        "tickMarkTint",
        "tickMarkTintMode",
        "canPerformGestures",
        "externalService",
        "supportsLocalInteraction",
        "startX",
        "startY",
        "endX",
        "endY",
        "offset",
        "use32bitAbi",
        "bitmap",
        "hotSpotX",
        "hotSpotY",
        "version",
        "backupInForeground",
        "countDown",
        "canRecord",
        "tunerCount",
        "fillType",
        "popupEnterTransition",
        "popupExitTransition",
        "forceHasOverlappingRendering",
        "contentInsetStartWithNavigation",
        "contentInsetEndWithActions",
        "numberPickerStyle",
        "enableVrMode",
        "UNKNOWN",
        "networkSecurityConfig",
        "shortcutId",
        "shortcutShortLabel",
        "shortcutLongLabel",
        "shortcutDisabledMessage",
        "roundIcon",
        "contextUri",
        "contextDescription",
        "showMetadataInPreview",
        "colorSecondary"
    ];

    // For now, we only care about the attribute names.
    id -= 0x1010000;

    Ok(attr_names[id as usize].to_string())
}
