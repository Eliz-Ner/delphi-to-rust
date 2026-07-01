mod serv_messages;
mod config;

use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use config::{ProjectSettings, load_config, save_config};
use slint::Model;

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, TabWidget, StandardTableView, LineEdit, CheckBox, ListView, GroupBox, Palette } from "std-widgets.slint";

    component ContextMenuItem {
        in property <string> text;
        callback clicked;

        min-height: 22px;

        states [
            hovered when touch-area.has-hover : {
                bg.background: Palette.accent-background;
                txt.color: Palette.accent-foreground;
            }
        ]

        bg := Rectangle {
            background: transparent;
            txt := Text {
                text: root.text;
                x: 15px;
                vertical-alignment: center;
                color: Palette.foreground;
                font-size: 11px;
            }
        }

        touch-area := TouchArea {
            clicked => { root.clicked(); }
        }
    }

    export component MainWindow inherits Window {
        callback choose_projects_source();
        callback choose_project_file();
        callback choose_chbase_source();
        callback save_settings();
        callback open_project();
        callback close_project();
        callback restart_project();
        callback delete_selected_project();
        callback set_autoload_for_selected(bool);
        callback set_allow_load_for_selected(bool);
        callback open_settings_dialog();
        callback prepare_settings_dialog();
        callback minimize_to_tray();
        callback allow_clients();
        callback disable_clients();
        callback reconnect_clients();
        callback restart_clients();
        callback close_clients();
        callback show_menu();

        callback add_server(string);
        callback delete_server();
        callback delete_log();
        callback add_log_files();
        callback add_chbase_node_from_folder();
        callback open_add_logs();
        callback open_import_logs();
        callback toggle_log_selection(int);
        callback commit_added_logs();
        callback check_nonexistent_logs();
        callback user_selected(int);
        callback update_user_level(int, string);

        title: "Монитор сервера";
        preferred-width: 800px;
        preferred-height: 550px;
        background: Palette.color-scheme == ColorScheme.dark ? #121214 : #e9edf3;
        default-font-size: 12px;

        in-out property <string> active_dialog: "none";
        in-out property <string> machine_name: "";
        in-out property <string> chbase_path: "";
        in-out property <bool> autoload_windows: false;
        in-out property <int> selected_project_index: -1;
        in-out property <int> selected_server_index: -1;
        in-out property <int> selected_log_index: -1;
        in-out property <int> selected_user_index: -1;
        in-out property <string> selected_user_level: "";
        in-out property <string> status_text: "";
        in-out property <bool> show_context_menu: false;
        in-out property <length> context_menu_x: 0px;
        in-out property <length> context_menu_y: 0px;
        in-out property <int> context_menu_project_index: -1;
        in-out property <bool> autoload_project_setting: false;
        in-out property <bool> allow_load_project_setting: false;
        in-out property <bool> is_restoring: false;
        in-out property <bool> show_info_dialog: false;
        in-out property <string> info_dialog_text: "";

        in-out property <[[StandardListViewItem]]> projects_data: [];
        in-out property <[[StandardListViewItem]]> chbase_main_data: [];
        in-out property <[[StandardListViewItem]]> clients_data: [];
        in-out property <[[StandardListViewItem]]> ppc_data: [];
        in-out property <[[StandardListViewItem]]> services_programs_data: [];
        in-out property <[[StandardListViewItem]]> servers_data: [];
        in-out property <[[StandardListViewItem]]> settings_projects_data: [];
        in-out property <[[StandardListViewItem]]> settings_users_data: [];
        in-out property <[[StandardListViewItem]]> settings_access_data: [];
        in-out property <[[StandardListViewItem]]> settings_chbase_data: [];
        in-out property <[[StandardListViewItem]]> services_data: [];
        in-out property <[[StandardListViewItem]]> logs_data: [];
        in-out property <[[StandardListViewItem]]> available_logs_data: [];

        changed selected_project_index => {
            root.prepare_settings_dialog();
        }

        changed selected_user_index => {
            root.user_selected(root.selected_user_index);
        }

        TouchArea {
            clicked => {
                root.selected-project-index = -1;
                root.show-context-menu = false;
            }
        }

        VerticalLayout {
            padding: 6px;
            spacing: 6px;

            Rectangle {
                background: Palette.color-scheme == ColorScheme.dark ? #1a1a1e : #ffffff;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 8px;
                vertical-stretch: 1;
                min-width: 0px;
                min-height: 0px;

                VerticalLayout {
                    padding: 8px;
                    spacing: 8px;

                    TabWidget {
                        vertical-stretch: 1;
                        min-height: 0px;

                        Tab {
                            title: "Проекты";
                            Rectangle {
                                background: Palette.color-scheme == ColorScheme.dark ? #222226 : #f8fafc;
                                border-color: Palette.border;
                                border-width: 1px;
                                border-radius: 6px;
                                min-height: 0px;
                                min-width: 0px;

                                touch_bg := TouchArea {
                                    width: 100%;
                                    height: 100%;
                                    pointer-event(event) => {
                                        if (event.button == PointerEventButton.right && event.kind == PointerEventKind.down) {
                                            root.selected-project-index = -1;
                                            root.context-menu-project-index = -1;
                                            root.context-menu-x = touch_bg.mouse-x;
                                            root.context-menu-y = touch_bg.mouse-y;
                                            root.show-context-menu = true;
                                        } else if (event.button == PointerEventButton.left && event.kind == PointerEventKind.down) {
                                            root.selected-project-index = -1;
                                            root.show-context-menu = false;
                                        }
                                    }

                                    StandardTableView {
                                        width: 100%;
                                        height: 100%;
                                        columns: [
                                            { title: "Имя проекта", width: 300px },
                                            { title: "Состояние" }
                                        ];
                                        rows: root.projects_data;
                                        current-row <=> root.selected_project_index;
                                    }
                                }
                            }
                        }

                        Tab {
                            title: "База каналов";
                            Rectangle {
                                background: Palette.color-scheme == ColorScheme.dark ? #222226 : #f8fafc;
                                border-color: Palette.border;
                                border-width: 1px;
                                border-radius: 6px;
                                min-height: 0px;
                                min-width: 0px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "ID", width: 50px },
                                        { title: "Состояние", width: 100px },
                                        { title: "Тип", width: 80px },
                                        { title: "Название", width: 140px },
                                        { title: "Имя Dll", width: 140px },
                                        { title: "Описание", width: 140px },
                                        { title: "Путь" }
                                    ];
                                    rows: root.chbase_main_data;
                                }
                            }
                        }
                    }

                    TabWidget {
                        vertical-stretch: 1;
                        min-height: 0px;

                        Tab {
                            title: "Рабочие станции";
                            Rectangle {
                                background: Palette.color-scheme == ColorScheme.dark ? #222226 : #f8fafc;
                                border-color: Palette.border;
                                border-width: 1px;
                                border-radius: 6px;
                                min-height: 0px;
                                min-width: 0px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя клиента", width: 130px },
                                        { title: "Имя пользователя", width: 160px },
                                        { title: "Время подключения", width: 160px },
                                        { title: "Имя машины", width: 160px },
                                        { title: "IP машины" }
                                    ];
                                    rows: root.clients_data;
                                }
                            }
                        }

                        Tab {
                            title: "Карманные компьютеры";
                            Rectangle {
                                background: Palette.color-scheme == ColorScheme.dark ? #222226 : #f8fafc;
                                border-color: Palette.border;
                                border-width: 1px;
                                border-radius: 6px;
                                min-height: 0px;
                                min-width: 0px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя клиента", width: 160px },
                                        { title: "Имя машины", width: 180px },
                                        { title: "Время подключения" }
                                    ];
                                    rows: root.ppc_data;
                                }
                            }
                        }

                        Tab {
                            title: "Сервисные программы";
                            Rectangle {
                                background: Palette.color-scheme == ColorScheme.dark ? #222226 : #f8fafc;
                                border-color: Palette.border;
                                border-width: 1px;
                                border-radius: 6px;
                                min-height: 0px;
                                min-width: 0px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя клиента", width: 160px },
                                        { title: "Имя машины", width: 180px },
                                        { title: "Время подключения" }
                                    ];
                                    rows: root.services_programs_data;
                                }
                            }
                        }

                        Tab {
                            title: "Сервера";
                            Rectangle {
                                background: Palette.color-scheme == ColorScheme.dark ? #222226 : #f8fafc;
                                border-color: Palette.border;
                                border-width: 1px;
                                border-radius: 6px;
                                min-height: 0px;
                                min-width: 0px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя сервера", width: 250px },
                                        { title: "Время подключения" }
                                    ];
                                    rows: root.servers_data;
                                }
                            }
                        }
                    }

                    Rectangle { height: 1px; background: Palette.border; vertical-stretch: 0; }

                    HorizontalLayout {
                        vertical-stretch: 0;
                        min-height: 30px;
                        spacing: 4px;
                        Button { text: "Настройки сервера"; clicked => { root.open_settings_dialog(); } }
                        Button { text: "Список серверов"; clicked => { root.active_dialog = "servers"; } }
                        Button { text: "Статистика"; clicked => { root.active_dialog = "stats"; } }
                        Button { text: "Настройка сервисов"; clicked => { root.active_dialog = "services"; } }
                        Button { text: "Протоколирование"; clicked => { root.active_dialog = "logs"; } }
                    }
                }
            }

            Rectangle {
                height: 28px;
                background: Palette.color-scheme == ColorScheme.dark ? #1a1a1e : #ffffff;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 8px;

                HorizontalLayout {
                    padding-left: 10px;
                    padding-right: 10px;
                    spacing: 20px;
                    Text { text: "Имя машины в сети: " + root.machine_name; vertical-alignment: center; font-weight: 600; color: Palette.foreground; }
                }
            }
        }

        if root.active_dialog != "none" : Rectangle {
            background: rgba(0, 0, 0, 0.45);
            TouchArea { clicked => { root.active_dialog = "none"; } }

            if root.active_dialog == "settings" : Rectangle {
                width: min(780px, root.width - 10px);
                height: min(530px, root.height - 10px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.color-scheme == ColorScheme.dark ? #1a1a1e : #ffffff;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;

                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 12px;
                    spacing: 10px;

                    Text { text: "Настройки сервера"; font-weight: 700; font-size: 14px; height: 22px; color: Palette.foreground; }

                    TabWidget {
                        vertical-stretch: 1;
                        Tab {
                            title: "Общие настройки сервера";
                            VerticalLayout {
                                width: 100%;
                                height: 100%;
                                padding: 12px;
                                spacing: 10px;
                                CheckBox { text: "Автозагрузка с Windows"; checked <=> root.autoload_windows; enabled: false; }
                                HorizontalLayout {
                                    spacing: 10px;
                                    Text { text: "Путь к базе каналов:"; vertical-alignment: center; width: 180px; color: Palette.foreground; }
                                    LineEdit { text <=> root.chbase_path; height: 24px; }
                                    Button { text: "..."; width: 30px; height: 24px; clicked => { root.choose_chbase_source(); } }
                                }
                            }
                        }

                        Tab {
                            title: "Настройки проектов";
                            HorizontalLayout {
                                width: 100%;
                                height: 100%;
                                padding: 8px;
                                spacing: 8px;
                                vertical-stretch: 1;

                                VerticalLayout {
                                    spacing: 8px;
                                    horizontal-stretch: 1;
                                    vertical-stretch: 1;
                                    Text { text: "Проекты:"; height: 20px; color: Palette.foreground; }
                                    Rectangle {
                                        background: Palette.background;
                                        border-color: Palette.border;
                                        border-width: 1px;
                                        border-radius: 6px;
                                        vertical-stretch: 1;
                                        min-height: 0px;
                                        min-width: 0px;
                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Проект", width: 240px },
                                                { title: "Автозагрузка", width: 120px },
                                                { title: "Разрешение" }
                                            ];
                                            rows: root.settings_projects_data;
                                            current-row <=> root.selected_project_index;
                                        }
                                    }
                                }

                                VerticalLayout {
                                    alignment: start;
                                    spacing: 8px;
                                    width: 220px;
                                    Text {
                                        text: root.selected_project_index >= 0 ? "Параметры проекта:" : "Проект не выбран"; 
                                        height: 20px;
                                        color: Palette.foreground;
                                        opacity: root.selected_project_index >= 0 ? 1.0 : 0.5;
                                        font-weight: 600;
                                    }
                                    CheckBox {
                                        text: "Автозагрузка";
                                        enabled: root.selected_project_index >= 0;
                                        checked <=> root.autoload_project_setting;
                                        toggled => {
                                            if (root.selected_project_index >= 0) {
                                                root.set_autoload_for_selected(self.checked);
                                            }
                                        }
                                    }
                                    CheckBox {
                                        text: "Разрешение загрузки";
                                        enabled: root.selected_project_index >= 0 && !root.autoload_project_setting;
                                        checked: root.autoload_project_setting || root.allow_load_project_setting;
                                        toggled => {
                                            if (root.selected_project_index >= 0 && !root.autoload_project_setting) {
                                                root.allow_load_project_setting = self.checked;
                                                root.set_allow_load_for_selected(self.checked);
                                            }
                                        }
                                    }
                                    Rectangle { height: 8px; }
                                    Button {
                                        text: "Добавить...";
                                        height: 24px;
                                        clicked => { root.choose_project_file(); }
                                    }
                                    Button {
                                        text: "Удалить";
                                        height: 24px;
                                        enabled: root.selected_project_index >= 0;
                                        clicked => { root.delete_selected_project(); }
                                    }
                                    Button {
                                        text: "Удалить всё";
                                        height: 24px;
                                        clicked => {
                                            root.settings_projects_data = [];
                                            root.selected_project_index = -1;
                                        }
                                    }
                                }
                            }
                        }

                        Tab {
                            title: "Права доступа";
                            VerticalLayout {
                                width: 100%;
                                height: 100%;
                                padding: 8px;
                                spacing: 8px;
                                vertical-stretch: 1;

                                HorizontalLayout {
                                    spacing: 8px;
                                    vertical-stretch: 1;
                                    VerticalLayout {
                                        spacing: 8px;
                                        width: 280px;
                                        vertical-stretch: 1;
                                        Text { text: "Имена пользователей:"; height: 20px; color: Palette.foreground; }
                                        Rectangle {
                                            background: Palette.background;
                                            border-color: Palette.border;
                                            border-width: 1px;
                                            border-radius: 6px;
                                            vertical-stretch: 1;
                                            min-height: 0px;
                                            min-width: 0px;
                                            StandardTableView {
                                                width: 100%;
                                                height: 100%;
                                                columns: [
                                                    { title: "Пользователь", width: 120px },
                                                    { title: "Ур. доступа", width: 80px },
                                                    { title: "Описание" }
                                                ];
                                                rows: root.settings_users_data;
                                                current-row <=> root.selected_user_index;
                                            }
                                        }
                                    }
                                    VerticalLayout {
                                        spacing: 10px;
                                        alignment: start;
                                        horizontal-stretch: 1;
                                        HorizontalLayout { spacing: 10px; Text { text: "Пароль:"; vertical-alignment: center; width: 120px; color: Palette.foreground; } LineEdit { height: 24px; } }
                                        HorizontalLayout {
                                            spacing: 10px;
                                            Text { text: "Уровень доступа:"; vertical-alignment: center; width: 120px; color: Palette.foreground; } 
                                            LineEdit {
                                                text <=> root.selected_user_level;
                                                height: 24px;
                                                width: 60px;
                                                enabled: root.selected_user_index >= 0;
                                                edited => { root.update_user_level(root.selected_user_index, self.text); }
                                            }
                                        }
                                        CheckBox { text: "Ограничение редактирования"; }
                                    }
                                }

                                HorizontalLayout {
                                    spacing: 8px;
                                    vertical-stretch: 1;
                                    VerticalLayout {
                                        width: 280px;
                                        spacing: 8px;
                                        vertical-stretch: 1;
                                        Text { text: "Проекты:"; height: 20px; color: Palette.foreground; }
                                        Rectangle {
                                            background: Palette.background;
                                            border-color: Palette.border;
                                            border-width: 1px;
                                            border-radius: 6px;
                                            vertical-stretch: 1;
                                            min-height: 0px;
                                            min-width: 0px;
                                            StandardTableView {
                                                width: 100%;
                                                height: 100%;
                                                columns: [
                                                    { title: "Проект", width: 180px },
                                                    { title: "Доступ" }
                                                ];
                                                rows: root.settings_access_data;
                                            }
                                        }
                                    }
                                    Rectangle {
                                        horizontal-stretch: 1;
                                        background: Palette.background;
                                        border-color: Palette.border;
                                        border-width: 1px;
                                        border-radius: 6px;
                                        min-height: 0px;
                                        min-width: 0px;
                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Имя пользователя", width: 150px },
                                                { title: "Ур. доступа", width: 90px },
                                                { title: "Машины" }
                                            ];
                                            rows: root.settings_access_data;
                                        }
                                    }
                                }
                            }
                        }

                        Tab {
                            title: "Настройка базы каналов";
                            HorizontalLayout {
                                width: 100%;
                                height: 100%;
                                padding: 12px;
                                spacing: 12px;
                                vertical-stretch: 1;
                                VerticalLayout {
                                    spacing: 8px;
                                    horizontal-stretch: 1;
                                    vertical-stretch: 1;
                                    Text { text: "Список узлов базы каналов:"; height: 20px; color: Palette.foreground; }
                                    Rectangle {
                                        background: Palette.background;
                                        border-color: Palette.border;
                                        border-width: 1px;
                                        border-radius: 6px;
                                        vertical-stretch: 1;
                                        min-height: 0px;
                                        min-width: 0px;
                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Узел", width: 200px },
                                                { title: "Можно загрузить" }
                                            ];
                                            rows: root.settings_chbase_data;
                                        }
                                    }
                                    Button {
                                        text: "Добавить...";
                                        height: 24px;
                                        clicked => { root.add_chbase_node_from_folder(); }
                                    }
                                    Button {
                                        text: "Удалить";
                                        height: 24px;
                                    }
                                    Button {
                                        text: "Удалить всё";
                                        height: 24px;
                                        clicked => { root.settings_chbase_data = []; }
                                    }
                                }
                                VerticalLayout {
                                    alignment: start;
                                    padding-top: 24px;
                                    CheckBox { text: "Разрешение загрузки узла базы каналов"; checked: true; }
                                    CheckBox { text: "Автозагрузка узлов"; }
                                }
                            }
                        }
                    }

                    HorizontalLayout {
                        alignment: end;
                        spacing: 10px;
                        height: 35px;
                        Button { text: "OK"; width: 80px; height: 24px; clicked => { root.save_settings(); root.active_dialog = "none"; } }
                        Button { text: "Отмена"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } }
                    }
                }
            }

            if root.active_dialog == "servers" : Rectangle {
                width: min(650px, root.width - 20px);
                height: min(340px, root.height - 20px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 12px;
                    spacing: 10px;
                    Text { text: "Список серверов"; font-weight: 700; height: 20px; color: Palette.foreground; }
                    HorizontalLayout {
                        spacing: 12px;
                        vertical-stretch: 1;
                        Rectangle {
                            horizontal-stretch: 1;
                            background: Palette.background;
                            border-color: Palette.border;
                            border-width: 1px;
                            border-radius: 6px;
                            min-height: 0px;
                            min-width: 0px;
                            StandardTableView {
                                width: 100%;
                                height: 100%;
                                columns: [
                                    { title: "Имя сервера", width: 180px },
                                    { title: "Время подключения" }
                                ];
                                rows: root.servers_data;
                                current-row <=> root.selected_server_index;
                            }
                        }
                        VerticalLayout {
                            alignment: start;
                            spacing: 10px;
                            width: 180px;
                            Text { text: "Имя сервера"; color: Palette.foreground; }
                            new_server_name := LineEdit { height: 24px; }
                            Button {
                                text: "Добавить сервер"; height: 24px;
                                clicked => {
                                    root.add_server(new_server_name.text);
                                    new_server_name.text = "";
                                }
                            }
                            Button {
                                text: "Удалить сервер"; height: 24px;
                                enabled: root.selected_server_index >= 0;
                                clicked => { root.delete_server(); }
                            }
                        }
                    }
                    HorizontalLayout { alignment: center; padding-top: 10px; height: 35px; spacing: 15px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Cancel"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "stats" : Rectangle {
                width: min(500px, root.width - 20px);
                height: 170px;
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 14px;
                    spacing: 12px;
                    Text { text: "Information"; font-weight: 700; color: Palette.foreground; }
                    HorizontalLayout {
                        spacing: 15px;
                        Text { text: "i"; font-size: 24px; color: Palette.accent-background; width: 24px; }
                        Text { text: "Connections = 0. NumCreateObj = 278, NumDeleteObj = 271, InitChbaseTags = 0"; wrap: word-wrap; color: Palette.foreground; }
                    }
                    HorizontalLayout { alignment: center; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "services" : Rectangle {
                width: min(520px, root.width - 20px);
                height: min(380px, root.height - 20px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 12px;
                    spacing: 10px;
                    Text { text: "Настройка списка списка сервисов"; font-weight: 700; height: 20px; color: Palette.foreground; }
                    Button { text: "Перезагрузка Dll сервисов"; height: 24px; clicked => { root.services_data = [ [ { text: "1 - buglog.dll" }, { text: "Перезагружен" } ], [ { text: "11 - PGUniServ.dll" }, { text: "Перезагружен" } ], [ { text: "12 - MSUniServ.dll" }, { text: "Перезагружен" } ], [ { text: "2 - ConnectionLog.dll" }, { text: "Перезагружен" } ], [ { text: "3 - propservice.dll" }, { text: "Перезагружен" } ], [ { text: "5 - ClientPLog.dll" }, { text: "Перезагружен" } ], [ { text: "6 - ClientTLog.dll" }, { text: "Перезагружен" } ] ]; } }
                    Rectangle {
                        background: Palette.background;
                        border-color: Palette.border;
                        border-width: 1px;
                        border-radius: 6px;
                        min-height: 0px;
                        min-width: 0px;
                        vertical-stretch: 1;
                        StandardTableView {
                            width: 100%;
                            height: 100%;
                            columns: [
                                { title: "Dll", width: 220px },
                                { title: "Состояние" }
                            ];
                            rows: root.services_data;
                        }
                    }
                    Button { text: "Настройка сервиса"; height: 24px; }
                    HorizontalLayout { alignment: end; spacing: 10px; height: 35px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Cancel"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "logs" : Rectangle {
                width: min(900px, root.width - 20px);
                height: min(510px, root.height - 20px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 12px;
                    spacing: 10px;
                    Text { text: "Настройка протоколирования"; font-weight: 700; height: 20px; color: Palette.foreground; }
                    HorizontalLayout {
                        spacing: 12px;
                        vertical-stretch: 1;
                        Rectangle {
                            horizontal-stretch: 1;
                            background: Palette.background;
                            border-color: Palette.border;
                            border-width: 1px;
                            border-radius: 6px;
                            min-height: 0px;
                            min-width: 0px;
                            StandardTableView {
                                width: 100%;
                                height: 100%;
                                columns: [
                                    { title: "№", width: 30px },
                                    { title: "Проект", width: 80px },
                                    { title: "Объект", width: 80px },
                                    { title: "Свойство", width: 80px },
                                    { title: "Тэг", width: 60px },
                                    { title: "ID канала", width: 80px },
                                    { title: "Дельта времени", width: 120px },
                                    { title: "Дельта", width: 60px },
                                    { title: "Всегда протоколировать" }
                                ];
                                rows: root.logs_data;
                                current-row <=> root.selected_log_index;
                            }
                        }
                        VerticalLayout {
                            width: 180px;
                            alignment: start;
                            spacing: 6px;
                            Button { text: "Добавить..."; height: 24px; clicked => { root.open_add_logs(); } }
                            Button {
                                text: "Удалить"; height: 24px;
                                enabled: root.selected_log_index >= 0;
                                clicked => { root.delete_log(); }
                            }
                            Button { text: "Удалить несуществующие"; height: 24px; clicked => { root.check_nonexistent_logs(); } }
                            Button { text: "Импорт протоколируемых"; height: 24px; clicked => { root.open_import_logs(); } }
                            Button {
                                text: "Добавить файлы..."; height: 24px;
                                clicked => { root.add_log_files(); }
                            }
                        }
                    }
                    HorizontalLayout { alignment: end; spacing: 10px; height: 35px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Отмена"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "add_logs" || root.active_dialog == "import_logs" : Rectangle {
                width: min(650px, root.width - 20px);
                height: min(400px, root.height - 20px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 12px;
                    spacing: 10px;
                    Text {
                        text: root.active_dialog == "add_logs" ? "Добавить..." : "Импорт..."; 
                        font-weight: 700; height: 20px; color: Palette.foreground;
                    }
                    Rectangle {
                        background: Palette.background;
                        border-color: Palette.border;
                        border-width: 1px;
                        border-radius: 6px;
                        min-height: 0px;
                        min-width: 0px;
                        vertical-stretch: 1;
                        StandardTableView {
                            width: 100%;
                            height: 100%;
                            columns: [
                                { title: "№", width: 40px },
                                { title: "Проект", width: 100px },
                                { title: "Объект", width: 100px },
                                { title: "Свойство", width: 100px },
                                { title: "Тэг", width: 60px },
                                { title: "ID канала", width: 100px },
                                { title: "Добавить" }
                            ];
                            rows: root.available_logs_data;
                            row-pointer-event(row, event, position) => {
                                if (event.kind == PointerEventKind.down) {
                                    root.toggle_log_selection(row);
                                }
                            }
                        }
                    }
                    HorizontalLayout {
                        alignment: center; spacing: 20px; height: 35px; padding-top: 10px;
                        Button { text: "Ок"; width: 80px; height: 24px; clicked => { root.commit_added_logs(); } }
                        Button { text: "Отмена"; width: 80px; height: 24px; clicked => { root.active_dialog = "logs"; } }
                    }
                }
            }
        }

        if root.show_info_dialog : Rectangle {
            background: rgba(0, 0, 0, 0.45);
            TouchArea { clicked => { root.show_info_dialog = false; } }

            Rectangle {
                width: 350px;
                height: 150px;
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    width: 100%;
                    height: 100%;
                    padding: 14px;
                    spacing: 12px;
                    Text { text: "Information"; font-weight: 700; height: 20px; color: Palette.foreground; }
                    HorizontalLayout {
                        spacing: 15px;
                        Text { text: "i"; font-size: 24px; color: Palette.accent-background; width: 24px; }
                        Text { text: root.info_dialog_text; wrap: word-wrap; color: Palette.foreground; }
                    }
                    HorizontalLayout { alignment: end; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.show_info_dialog = false; } } }
                }
            }
        }

        if root.show_context_menu : Rectangle {
            background: rgba(0, 0, 0, 0.02);
            TouchArea { clicked => { root.show_context_menu = false; } }

            Rectangle {
                x: root.context_menu_x;
                y: root.context_menu_y;
                width: 220px;
                background: Palette.background;
                border-color: Palette.border;
                border-width: 1px;
                border-radius: 2px;

                VerticalLayout {
                    padding: 2px;
                    spacing: 1px;

                    ContextMenuItem {
                        text: "Открыть проект";
                        clicked => {
                            root.open_project();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Закрыть проект";
                        clicked => {
                            root.close_project();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Перезагрузить проект";
                        clicked => {
                            root.restart_project();
                            root.show_context_menu = false;
                        }
                    }

                    Rectangle {
                        height: 7px;
                        VerticalLayout {
                            padding-top: 3px;
                            padding-bottom: 3px;
                            Rectangle {
                                height: 1px;
                                background: Palette.border;
                            }
                        }
                    }

                    ContextMenuItem {
                        text: "Приостановить выполнение";
                        clicked => {
                            root.show_context_menu = false;
                        }
                    }

                    Rectangle {
                        height: 7px;
                        VerticalLayout {
                            padding-top: 3px;
                            padding-bottom: 3px;
                            Rectangle {
                                height: 1px;
                                background: Palette.border;
                            }
                        }
                    }

                    ContextMenuItem {
                        text: "Запретить подключение клиентов";
                        clicked => {
                            root.allow_clients();
                            root.show_context_menu = false;
                        }
                    }

                    Rectangle {
                        height: 7px;
                        VerticalLayout {
                            padding-top: 3px;
                            padding-bottom: 3px;
                            Rectangle {
                                height: 1px;
                                background: Palette.border;
                            }
                        }
                    }

                    ContextMenuItem {
                        text: "Подключить клиентов";
                        clicked => {
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Отключить клиентов";
                        clicked => {
                            root.disable_clients();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Переподключить клиентов";
                        clicked => {
                            root.reconnect_clients();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Перезагрузить клиентов";
                        clicked => {
                            root.restart_clients();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Закрыть клиентов";
                        clicked => {
                            root.close_clients();
                            root.show_context_menu = false;
                        }
                    }
                }
            }
        }
    }
}

type RowModel = slint::ModelRc<slint::StandardListViewItem>;

fn file_name_text(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .unwrap_or_else(|| path.display().to_string())
}

fn make_row(items: &[&str]) -> RowModel {
    slint::ModelRc::new(slint::VecModel::from(
        items.iter().map(|item| slint::StandardListViewItem::from(*item)).collect::<Vec<_>>()
    ))
}

fn main() -> Result<(), slint::PlatformError> {
    // Конфигурация
    let config = Rc::new(RefCell::new(load_config()));

    let ui = MainWindow::new()?;
    let weak_ui = ui.as_weak();

    // Трей
    let tray_result = tray_item::TrayItem::new("MonitorServer", "icon");
    let mut tray = match tray_result {
        Ok(t) => Some(t),
        Err(_) => None
    };

    if let Some(ref mut t) = tray {
        let w_open = weak_ui.clone();
        let _ = t.add_menu_item("Open", move || {
            let inner_open = w_open.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(win) = inner_open.upgrade() {
                    win.set_is_restoring(true);
                    let _ = win.show();

                    let win_weak = inner_open.clone();
                    slint::Timer::single_shot(std::time::Duration::from_millis(1500), move || {
                        if let Some(w) = win_weak.upgrade() {
                            w.set_is_restoring(false);
                        }
                    });
                }
            });
        });

        let _ = t.add_menu_item("Exit", move || {
            std::process::exit(0);
        });
    }

    // Выход
    ui.window().on_close_requested(move || {
        std::process::exit(0);
    });

    // Сворачивание
    let ui_handle = weak_ui.clone();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(250), move || {
        if let Some(ui) = ui_handle.upgrade() {
            if ui.window().is_minimized() && !ui.get_is_restoring() {
                let _ = ui.window().hide();
                ui.window().set_minimized(false);
            }
        }
    });

    // Переменные
    let project_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));
    let settings_project_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));
    let servers_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));
    let logs_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));
    let available_logs_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));
    let settings_chbase_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));

    let settings_users_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(vec![
        make_row(&["SUPERVISOR", "10", ""]),
        make_row(&["USER", "1", ""]),
    ]));
    ui.set_settings_users_data(slint::ModelRc::new(slint::VecModel::from(settings_users_rows.borrow().clone())));

    // Параметры
    {
        let current_cfg = config.borrow();
        ui.set_autoload_windows(current_cfg.autoload_windows);
        ui.set_machine_name(current_cfg.machine_name.clone().into());
        ui.set_chbase_path(current_cfg.chbase_path.clone().into());

        // Проекты
        for project in &current_cfg.projects {
            let autoload_str = if project.autoload { "Да" } else { "Нет" };
            let allow_str = if project.allow_load { "Да" } else { "Нет" };

            settings_project_rows.borrow_mut().push(make_row(&[&project.name, autoload_str, allow_str]));

            if project.autoload {
                project_rows.borrow_mut().push(make_row(&[&project.name, "Остановлен"]));
            }
        }
    }

    ui.set_projects_data(slint::ModelRc::new(slint::VecModel::from(project_rows.borrow().clone())));
    ui.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(settings_project_rows.borrow().clone())));

    ui.on_open_settings_dialog({
        let weak_ui = weak_ui.clone();
        let config_rc = config.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let cfg = config_rc.borrow();

            let mut sp_rows = settings_project_rows.borrow_mut();
            sp_rows.clear();
            for project in &cfg.projects {
                let autoload_str = if project.autoload { "Да" } else { "Нет" };
                let allow_str = if project.allow_load { "Да" } else { "Нет" };
                sp_rows.push(make_row(&[&project.name, autoload_str, allow_str]));
            }
            window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(sp_rows.clone())));
            window.set_autoload_windows(cfg.autoload_windows);
            window.set_chbase_path(cfg.chbase_path.clone().into());

            window.set_selected_user_index(-1);
            window.set_selected_user_level("".into());

            window.invoke_prepare_settings_dialog();
            window.set_active_dialog("settings".into());
        }
    });

    ui.on_user_selected({
        let weak_ui = weak_ui.clone();
        let users_rows = settings_users_rows.clone();
        move |idx| {
            if let Some(window) = weak_ui.upgrade() {
                let rows = users_rows.borrow();
                if idx >= 0 && (idx as usize) < rows.len() {
                    let level = rows[idx as usize].row_data(1).map(|it| it.text.to_string()).unwrap_or_default();
                    window.set_selected_user_level(level.into());
                } else {
                    window.set_selected_user_level("".into());
                }
            }
        }
    });

    ui.on_update_user_level({
        let weak_ui = weak_ui.clone();
        let users_rows = settings_users_rows.clone();
        move |idx, level| {
            if let Some(window) = weak_ui.upgrade() {
                let mut rows = users_rows.borrow_mut();
                if idx >= 0 && (idx as usize) < rows.len() {
                    let name = rows[idx as usize].row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                    let desc = rows[idx as usize].row_data(2).map(|it| it.text.to_string()).unwrap_or_default();
                    rows[idx as usize] = make_row(&[&name, level.as_str(), &desc]);
                    window.set_settings_users_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
                }
            }
        }
    });

    ui.on_add_chbase_node_from_folder({
        let weak_ui = weak_ui.clone();
        let chbase_nodes_rows = settings_chbase_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                let folder_name = file_name_text(&path);
                let mut rows = chbase_nodes_rows.borrow_mut();
                rows.push(make_row(&[&folder_name, "Да"]));
                window.set_settings_chbase_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
            }
        }
    });

    ui.on_choose_projects_source({
        let weak_ui = weak_ui.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let mut dialog = rfd::FileDialog::new();
            let chbase_path = window.get_chbase_path();
            let chbase_path = Path::new(&chbase_path);
            if chbase_path.exists() {
                dialog = dialog.set_directory(chbase_path);
            }
            let Some(path) = dialog.pick_folder() else { return; };
            let folder_name = file_name_text(&path);
            let mut entries = Vec::new();
            if let Ok(read_dir) = fs::read_dir(&path) {
                for entry in read_dir.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        entries.push(name.to_string());
                    }
                }
            }
            window.set_chbase_path(path.display().to_string().into());
            let mut rows: Vec<RowModel> = Vec::new();
            if entries.is_empty() {
                rows.push(make_row(&[folder_name.as_str(), "Да"]));
            } else {
                for name in entries {
                    rows.push(make_row(&[name.as_str(), "Да"]));
                }
            }
            window.set_settings_chbase_data(slint::ModelRc::new(slint::VecModel::from(rows)));
        }
    });

    ui.on_choose_project_file({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let mut dialog = rfd::FileDialog::new();
            let chbase_path = window.get_chbase_path();
            let chbase_path = Path::new(&chbase_path);
            if chbase_path.exists() {
                dialog = dialog.set_directory(chbase_path);
            }
            let Some(path) = dialog.pick_file() else { return; };
            let file_name = file_name_text(&path);
            let new_settings = {
                let mut s_rows = settings_project_rows.borrow_mut();
                s_rows.push(make_row(&[file_name.as_str(), "Нет", "Нет"]));
                s_rows.clone()
            };
            window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
        }
    });

    ui.on_delete_selected_project({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_settings = {
                    let mut s_rows = settings_project_rows.borrow_mut();
                    let removed_name_opt = s_rows.get(idx_usize).and_then(|r| r.row_data(0)).map(|it| it.text.to_string());
                    if removed_name_opt.is_some() {
                        s_rows.remove(idx_usize);
                    }
                    s_rows.clone()
                };
                window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
                window.set_selected_project_index(-1);
            }
        }
    });

    ui.on_set_autoload_for_selected({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move |flag: bool| {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_settings = {
                    let mut s_rows = settings_project_rows.borrow_mut();
                    let (name, allow) = {
                        if let Some(row) = s_rows.get(idx_usize) {
                            let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                            let allow = row.row_data(2).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                            (name, allow)
                        } else { return; }
                    };
                    let autoload_str = if flag { "Да" } else { "Нет" };
                    let allow_str = if allow { "Да" } else { "Нет" };
                    s_rows[idx_usize] = make_row(&[name.as_str(), autoload_str, allow_str]);
                    s_rows.clone()
                };
                window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
            }
        }
    });

    ui.on_set_allow_load_for_selected({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move |flag: bool| {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_settings = {
                    let mut s_rows = settings_project_rows.borrow_mut();
                    let (name, autoload) = {
                        if let Some(row) = s_rows.get(idx_usize) {
                            let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                            let autoload = row.row_data(1).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                            (name, autoload)
                        } else { return; }
                    };
                    let autoload_str = if autoload { "Да" } else { "Нет" };
                    let allow_str = if flag { "Да" } else { "Нет" };
                    s_rows[idx_usize] = make_row(&[name.as_str(), autoload_str, allow_str]);
                    s_rows.clone()
                };
                window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
            }
        }
    });

    ui.on_prepare_settings_dialog({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let s_rows = settings_project_rows.borrow();
                if let Some(row) = s_rows.get(idx_usize) {
                    let autoload = row.row_data(1).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                    let allow = row.row_data(2).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                    window.set_autoload_project_setting(autoload);
                    window.set_allow_load_project_setting(allow);
                } else {
                    window.set_autoload_project_setting(false);
                    window.set_allow_load_project_setting(false);
                }
            } else {
                window.set_autoload_project_setting(false);
                window.set_allow_load_project_setting(false);
            }
        }
    });

    ui.on_save_settings({
        let weak_ui = weak_ui.clone();
        let config_rc = config.clone();
        let project_rows = project_rows.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let mut cfg = config_rc.borrow_mut();
            cfg.autoload_windows = window.get_autoload_windows();
            cfg.chbase_path = window.get_chbase_path().to_string();

            let mut projects = Vec::new();
            let selected_idx = window.get_selected_project_index();
            {
                let s_rows = settings_project_rows.borrow();
                for (i, row_model) in s_rows.iter().enumerate() {
                    let row = row_model.clone();
                    if let Some(name_item) = row.row_data(0) {
                        let name = name_item.text.to_string();
                        let (autoload, allow_load) = if (i as i32) == selected_idx {
                            (window.get_autoload_project_setting(), window.get_allow_load_project_setting())
                        } else {
                            (row.row_data(1).map(|item| item.text.to_string() == "Да").unwrap_or(false),
                             row.row_data(2).map(|item| item.text.to_string() == "Да").unwrap_or(false))
                        };
                        projects.push(ProjectSettings { name, autoload, allow_load, available_for_clients: false });
                    }
                }
            }
            cfg.projects = projects;
            let _ = save_config(&cfg);

            let new_projects = {
                let mut p_rows = project_rows.borrow_mut();
                p_rows.clear();
                for project in &cfg.projects {
                    if project.autoload {
                        p_rows.push(make_row(&[&project.name, "Остановлен"]));
                    }
                }
                p_rows.clone()
            };
            window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_projects)));
        }
    });

    ui.on_open_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_context_menu_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_rows = {
                    let mut p_rows = project_rows.borrow_mut();
                    if let Some(row) = p_rows.get(idx_usize) {
                        let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                        p_rows[idx_usize] = make_row(&[name.as_str(), "Работает"]);
                    }
                    p_rows.clone()
                };
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_rows)));
            }
        }
    });

    ui.on_close_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_context_menu_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_rows = {
                    let mut p_rows = project_rows.borrow_mut();
                    if let Some(row) = p_rows.get(idx_usize) {
                        let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                        p_rows[idx_usize] = make_row(&[name.as_str(), "Остановлен"]);
                    }
                    p_rows.clone()
                };
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_rows)));
            }
        }
    });

    ui.on_restart_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_context_menu_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_rows = {
                    let mut p_rows = project_rows.borrow_mut();
                    if let Some(row) = p_rows.get(idx_usize) {
                        let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                        p_rows[idx_usize] = make_row(&[name.as_str(), "Перезапуск"]);
                    }
                    p_rows.clone()
                };
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_rows)));
            }
        }
    });

    ui.on_add_server({
        let weak_ui = weak_ui.clone();
        let servers_rows = servers_rows.clone();
        move |name: slint::SharedString| {
            let Some(window) = weak_ui.upgrade() else { return; };
            let name_str = name.to_string();
            if name_str.is_empty() { return; }
            let now = chrono::Local::now().format("%d.%m.%Y %H:%M:%S").to_string();
            let mut rows = servers_rows.borrow_mut();
            rows.push(make_row(&[&name_str, &now]));
            window.set_servers_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
        }
    });

    ui.on_delete_server({
        let weak_ui = weak_ui.clone();
        let servers_rows = servers_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_server_index();
            if idx >= 0 {
                let mut rows = servers_rows.borrow_mut();
                if (idx as usize) < rows.len() {
                    rows.remove(idx as usize);
                }
                window.set_servers_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
                window.set_selected_server_index(-1);
            }
        }
    });

    ui.on_delete_log({
        let weak_ui = weak_ui.clone();
        let logs_rows = logs_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_log_index();
            if idx >= 0 {
                let mut rows = logs_rows.borrow_mut();
                if (idx as usize) < rows.len() {
                    rows.remove(idx as usize);
                }
                window.set_logs_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
                window.set_selected_log_index(-1);
            }
        }
    });

    ui.on_add_log_files({
        let weak_ui = weak_ui.clone();
        let logs_rows = logs_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let files = rfd::FileDialog::new().pick_files();
            if let Some(paths) = files {
                let mut rows = logs_rows.borrow_mut();
                for path in paths {
                    let fname = file_name_text(&path);
                    let id = format!("{}", rows.len() + 1);
                    rows.push(make_row(&[&id, &fname, "Object", "Value", "Tag", "1", "00:00:00", "0.0", "False"]));
                }
                window.set_logs_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
            }
        }
    });

    ui.on_open_add_logs({
        let weak_ui = weak_ui.clone();
        let available_logs = available_logs_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let mut rows = available_logs.borrow_mut();
            rows.clear();
            rows.push(make_row(&["1", "AppComb", "C11LS1", "State", "☑", "0100005D", "☐"]));
            rows.push(make_row(&["2", "AppComb", "C11LS2", "State", "☑", "0100005E", "☐"]));
            rows.push(make_row(&["3", "AppComb", "C11N1", "State", "☑", "01000038", "☐"]));
            rows.push(make_row(&["4", "AppComb", "C11V1", "State", "☑", "01000047", "☐"]));
            window.set_available_logs_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
            window.set_active_dialog("add_logs".into());
        }
    });

    ui.on_open_import_logs({
        let weak_ui = weak_ui.clone();
        let available_logs = available_logs_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let mut rows = available_logs.borrow_mut();
            rows.clear();
            rows.push(make_row(&["1", "Alkali", "LT1", "V_Recalc", "☐", "00000000", "☐"]));
            rows.push(make_row(&["2", "Alkali", "TANK10V4", "ladd", "☐", "00000000", "☐"]));
            window.set_available_logs_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
            window.set_active_dialog("import_logs".into());
        }
    });

    ui.on_toggle_log_selection({
        let weak_ui = weak_ui.clone();
        let available_logs = available_logs_rows.clone();
        move |idx| {
            let Some(window) = weak_ui.upgrade() else { return; };
            let mut rows = available_logs.borrow_mut();
            if idx >= 0 && (idx as usize) < rows.len() {
                let mut r_data: Vec<String> = vec![];
                for i in 0..7 {
                    r_data.push(rows[idx as usize].row_data(i).map(|it| it.text.to_string()).unwrap_or_default());
                }
                if r_data[6] == "☐" {
                    r_data[6] = "☑".to_string();
                } else {
                    r_data[6] = "☐".to_string();
                }
                let str_refs: Vec<&str> = r_data.iter().map(|s| s.as_str()).collect();
                rows[idx as usize] = make_row(&str_refs);
                window.set_available_logs_data(slint::ModelRc::new(slint::VecModel::from(rows.clone())));
            }
        }
    });

    ui.on_commit_added_logs({
        let weak_ui = weak_ui.clone();
        let available_logs = available_logs_rows.clone();
        let logs_rows = logs_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let a_rows = available_logs.borrow();
            let mut l_rows = logs_rows.borrow_mut();
            for row in a_rows.iter() {
                let checkbox = row.row_data(6).map(|it| it.text.to_string()).unwrap_or_default();
                if checkbox == "☑" {
                    let id = format!("{}", l_rows.len() + 1);
                    let proj = row.row_data(1).map(|it| it.text.to_string()).unwrap_or_default();
                    let obj = row.row_data(2).map(|it| it.text.to_string()).unwrap_or_default();
                    let prop = row.row_data(3).map(|it| it.text.to_string()).unwrap_or_default();
                    let tag = row.row_data(4).map(|it| it.text.to_string()).unwrap_or_default();
                    let ch_id = row.row_data(5).map(|it| it.text.to_string()).unwrap_or_default();
                    l_rows.push(make_row(&[&id, &proj, &obj, &prop, &tag, &ch_id, "0", "0", "☐"]));
                }
            }
            window.set_logs_data(slint::ModelRc::new(slint::VecModel::from(l_rows.clone())));
            window.set_active_dialog("logs".into());
        }
    });

    ui.on_check_nonexistent_logs({
        let weak_ui = weak_ui.clone();
        move || {
            if let Some(window) = weak_ui.upgrade() {
                window.set_info_dialog_text("На данный момент нет несуществующих свойств в проектах.".into());
                window.set_show_info_dialog(true);
            }
        }
    });

    ui.on_allow_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_disable_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_reconnect_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_restart_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_close_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_show_menu({
        let weak_ui = weak_ui.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                window.set_show_context_menu(true);
                window.set_context_menu_x(100.0);
                window.set_context_menu_y(100.0);
                window.set_context_menu_project_index(idx);
            }
        }
    });

    ui.show()?;

    // Цикл
    slint::run_event_loop_until_quit()?;
    Ok(())
}