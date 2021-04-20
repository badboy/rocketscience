// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!



#ifndef mozilla_dom_Rocket
#define mozilla_dom_Rocket

#include "jsapi.h"
#include "nsCOMPtr.h"
#include "nsIGlobalObject.h"
#include "nsWrapperCache.h"

#include "mozilla/RefPtr.h"

#include "mozilla/dom/RocketscienceBinding.h"

namespace mozilla {
namespace dom {

class Rocket final : public nsISupports, public nsWrapperCache {
 public:
  NS_DECL_CYCLE_COLLECTING_ISUPPORTS
  NS_DECL_CYCLE_COLLECTION_SCRIPT_HOLDER_CLASS(Rocket)

  Rocket(nsIGlobalObject* aGlobal, uint64_t aHandle);

  JSObject* WrapObject(JSContext* aCx,
                       JS::Handle<JSObject*> aGivenProto) override;

  nsIGlobalObject* GetParentObject() const { return mGlobal; }

  static already_AddRefed<Rocket> Constructor(
    GlobalObject& aGlobal,
    const nsAString& name
  );

  void Show(
    nsAString& aRetVal
  );

  bool Launch(
    ErrorResult& aRv
  );

  void Add(
    const Part& part
  );

  void LockSteering(
    Direction direction
  );

 private:
  ~Rocket();

  nsCOMPtr<nsIGlobalObject> mGlobal;
  uint64_t mHandle;
};

}  // namespace dom
}  // namespace mozilla

#endif  // mozilla_dom_Rocket