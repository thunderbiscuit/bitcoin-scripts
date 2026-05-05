import pkg_resources
import warnings

def check_bdk_version(original_major_version, original_minor_version, package_name="bdkpython"):
    original_version = pkg_resources.parse_version(f"{original_major_version}.{original_minor_version}")
    installed_version = pkg_resources.get_distribution(package_name).version
    installed_version_without_patch = pkg_resources.parse_version(".".join(installed_version.split(".")[:2]))

    # warning: v0.25 vs. v0.26
    # no warning: v0.25 vs v0.25.2
    if original_version != installed_version_without_patch:
        text = f"\n\n!!!\nThis script was written with {package_name} v{original_version}, but the installed version is v{installed_version} which may cause errors.\n!!!\n"
        warnings.warn(text)
