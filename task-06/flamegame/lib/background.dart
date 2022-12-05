import 'package:flame/components.dart';
    
    class Background extends SpriteComponent with HasGameRef {
      @override
      Future<void> onLoad() async {
        super.onLoad();
        sprite = await gameRef.loadSprite('background.png');
        size = sprite!.originalSize;
      }
    }
    