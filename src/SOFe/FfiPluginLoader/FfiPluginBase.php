<?php

namespace SOFe\FfiPluginLoader;

use pocketmine\plugin\PluginBase;

final class FfiPluginBase extends PluginBase {
	public function onEnable() {
		$ffi = FfiPluginLoader::loadSo($this->getFile());
		$ffi->plugin_main();
	}
}
