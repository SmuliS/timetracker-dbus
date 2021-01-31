%define debug_package %{nil}
%define _preset_priority 90

Name:       timetracker-dbus
Version:    1
Release:    1
Summary:    Simple D-Bus Server to communicate timetracker events.
License:    MIT
Source0:    %{name}-%{_commit}.tar.gz

BuildRequires: systemd-rpm-macros
%{systemd_requires}

%description
D-Bus server to integrate with e.g. timetracker gnome extension

%prep
%setup -n %{name}-%{_commit}

%build
cargo build --release

# Systemd unit
cat > %{name}.service <<EOF
[Unit]
Description=Simple D-Bus Server to communicate timetracker events.

[Service]
ExecStart=/usr/local/bin/timetracker-dbus

[Install]
WantedBy=multi-user.target
EOF

# Systemd preset
cat > 90-%{name}.preset <<EOF
enable %{name}.service
EOF

%install
install -Dm 755 target/release/%{name}               %{buildroot}/usr/local/bin/%{name}
install -Dm 755 %{name}.service                      %{buildroot}%{_userunitdir}/%{name}.service
install -Dm 755 %{_preset_priority}-%{name}.preset   %{buildroot}%{_userpresetdir}/%{_preset_priority}-%{name}.preset

%files
/usr/local/bin/%{name}
%{_userunitdir}/%{name}.service
%{_userpresetdir}/%{_preset_priority}-%{name}.preset

%post
%systemd_user_post %{name}.service

%preun
%systemd_user_preun %{name}.service

%postun
%systemd_user_postun %{name}.service
%systemd_postun_with_restart %{name}.service
